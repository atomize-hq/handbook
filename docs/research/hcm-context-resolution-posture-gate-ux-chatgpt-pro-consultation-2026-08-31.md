# HCM Context Resolution, posture, gate, and UX ChatGPT Pro consultation

Date: 2026-08-31

Task: fresh independent advisory review of the two proposed HCM semantic decisions
against the current Context Resolution, Snapshot/Grounding, Contracts, Dock,
CLI/adoption, user-experience, workflow, and future host-enforcement boundaries.

Current approach: preserve the proposed posture -> assurance requirements ->
evaluation -> metric/gate -> downstream enforcement decomposition; pause before
writing another decision record or beginning implementation planning; determine
whether Context Resolution and explanation projection require bounded amendments
or separate decisions.

Model surface: ChatGPT Pro account, Chat surface, Extra High reasoning. The
current UI did not expose a more specific underlying model name.

Conversation: https://chatgpt.com/c/6a96368c-d814-83ea-b21d-7394e00f178f

Advisory only; verify against local project truth and authoritative docs; do not
reduce scope without user approval.

## Sources supplied

The fresh conversation received these complete repository files:

1. `docs/specs/handbook-contract-membrane/metric-gate-semantic-decision.md`
2. `docs/specs/handbook-contract-membrane/posture-assurance-policy-semantic-decision.md`
3. `docs/specs/handbook-contract-membrane/architecture-context.md`
4. `docs/specs/handbook-contract-membrane/implemented-baseline.md`
5. `docs/specs/handbook-contract-membrane/phase-and-slice-map.md`
6. `docs/specs/handbook-contract-membrane/slices/HCM-4.6/README.md`
7. `docs/specs/handbook-contract-membrane/slices/HCM-5.2/README.md`
8. `crates/engine/definitions/context-resolution/handbook.context-resolution.shipped-root/1.0.0.yaml`

No prior ChatGPT review or conversation was supplied.

## Prompt sent

~~~text
We are already working on this project task.

Task/spec: Independently review the proposed Handbook Contract Membrane metric/evaluation/gate and posture/assurance-policy semantic decisions in light of the existing Context Resolution system. Identify semantic, ownership, integration, user-experience, and workflow gaps exposed by the questions below. We are deliberately pausing before writing another decision record or beginning implementation planning. We want guidance on whether the two proposed decisions remain sound, which additional decisions are truly necessary, and how to avoid creating overlapping meanings for Resolution, posture, assurance, evidence sufficiency, gate truth, explanation density, and runtime enforcement. This is an advisory architecture/product review, not a request for code or a patch.

Current approach: Two complete proposed greenfield decisions are attached: metric-gate-semantic-decision.md and posture-assurance-policy-semantic-decision.md. They have already been cross-aligned locally, but this is a fresh independent review; do not assume or rely on any prior ChatGPT conversations or reviews. The current direction is to keep evaluation truth canonical and closed, let exact resolved posture select exact assurance requirements without changing metric/gate meaning, and potentially add a separate user-facing explanation projection so lower-density views present the same evaluation rather than performing a smaller or different evaluation. Repository-derived inference: that additional projection decision may be needed, but it has not been written or approved. We are specifically asking whether to keep, adjust, or pivot before doing so.

Project sources: Read every attachment before answering. Read architecture-context.md and implemented-baseline.md first for authority and current-state labels; then 1.0.0.yaml for the shipped Context Resolution vocabulary; then the two README.md attachments, identifiable by their headings as the completed HCM-4.6 snapshot/grounding slice and completed HCM-5.2 evidence/verdict/gate slice; then phase-and-slice-map.md for adoption and future-consumer boundaries; finally read both complete proposed semantic decisions together. Treat source and tests as product truth, current architecture/baseline as repository context, completed slice documents as bounded outcome evidence, and the two decision records as proposals rather than implementation authority. Do not use prior review documents or infer excluded repository content.

Evidence, blockers, constraints, and validation: The following is a source-derived technical précis that should be checked against the attachments and treated as repository fact only where the attached sources support it.

1. Engine publicly owns the Context Resolution kernel and registered stack semantics. The shipped stack has four presets (strategic, coordination, execution, operation) and six independently ranked dimensions: scope_horizon, detail_resolution, temporal_horizon, authority_horizon, memory_horizon, and validation_horizon. The full Engine ContextResolutionEnvelope additionally carries active level, exact profile/stack bindings, authority admission, constraints, mutation rules, parent identity, and escalation inputs. It supports root resolution, child non-expansion, repository-path mutation decisions, memory authorization/promotion, validation authorization, and escalation candidates.

2. The Engine Context Resolution kernel is public. What remains private/test-oriented is the generic deterministic projection executor—not Resolution itself. There is no universal production service through which every Handbook artifact or rendering passes.

3. Ordinary adoption is narrow. Production Snapshot/Grounding performs a Resolution- and disclosure-bounded projection with lossiness, omissions, refusals, rechecks, exact stack identity, and all six values. Ordinary artifact intake/normalization/preview/promotion/read, Charter/posture rendering, metric/gate explanation rendering, and general CLI/UI rendering do not automatically use Context Resolution. The full execution envelope is an implemented kernel capability but does not currently wrap ordinary Codex task execution or every CLI/tool effect.

4. Contracts does not implement a competing Resolution ranking system. It owns a durable/serializable ResolutionEnvelopeV1 containing an exact stack binding and six coordinates, plus per-claim ResolutionMinimumV1. Contracts converts those coordinates into Engine ResolutionObservationV1 values and delegates dimensionwise minimum and meets-minimum comparison to Engine's registered stack semantics. Contracts owns the evidence/claim consequence; Engine owns value meaning and order. Dock reuses the Contracts representation to bind requested Resolution, actual Resolution, and a producer ceiling.

5. Current evidence behavior is deliberately fail-closed: effective Resolution is the dimensionwise minimum of requested, actual, and optional producer ceiling. Evidence below an observed claim's minimum is refused during admission; verdict evaluation rechecks effective Resolution as defense in depth. A blocked/not-observed required or hard-fail claim then blocks the gate; an advisory claim has separately defined deficit behavior. Thus evidence Resolution legitimately affects gate truth through claim eligibility. It must not be confused with user-facing explanation density.

6. A local source inspection found no run-preparation comparison that preflights the requested evaluation Resolution against every known applicable claim minimum. Repository-derived inference: an inherently underpowered run may be constructible, then fail later at candidate admission or surface as missing/not-observed evidence in a gate. This may be intentional if diagnostic evaluations are allowed, or a seam defect if every prepared run is expected to be gate-capable. Please assess this explicitly and distinguish verified implications from assumptions.

7. The label detail_resolution currently belongs to evidence/execution capability. Its values identifier_only, summary, normal, and full are tempting to reuse as presentation modes, creating a serious vocabulary and coupling risk. A user preference such as “show me the simple version” must not lower evidence eligibility, change the requested evaluation Resolution, change claim findings, reselect assurance requirements, or change gate truth. Any explanation projection should occur after one canonical evaluation. Lower-density output must preserve at least the decision, decisive blockers/reasons, material unknowns/refusals, selected posture/assurance basis when relevant, and an authorized drill-down path. The detailed view must be derivable from the same canonical result, not regenerated as a new interpretation.

8. The CLI currently exposes generic bootstrap, catalog-read, schema-read, and invoke behavior. An owner operation invoked through Handbook can enforce its own schema, authority, currentness, mutation, and semantic rules. But merely feeding an agent a Context Resolution envelope through a prompt or CLI does not constrain an unrestricted shell or unrelated tool. Task-wide enforcement requires a host/capability broker. Current Substrate publication/bridge/adoption is external-unknown, not implemented repository truth. A future Substrate integration could enforce tool availability, writable paths/mounts, credentials, memory access, network/process policy where supported, and provenance of actual effects; Handbook should remain the semantic owner while Substrate translates admitted policy into host capabilities. Not every dimension is an OS restriction: temporal/detail/validation may primarily constrain disclosure, evidence admission, or accepted claims.

9. The proposed posture decision defines a nine-dimensional scoped posture vector and immutable Resolved Posture Snapshot, followed by exact Assurance Policy selection and Resolved Assurance Requirements. It deliberately separates Project Classification, posture, exception authority, evidence policy, metrics, gates, enforcement, recommendation, and transition. The metric/gate decision defines exact named metric algorithms, closed evaluation contexts, four semantic gate results, complete canonical explanation/accounting, and downstream non-authority. Please check whether Context Resolution is correctly placed relative to this chain: whether posture/assurance policy should select evaluation Resolution requirements or minima; whether Context Resolution should be another exact input/output of Resolved Assurance Requirements; whether those concepts must remain orthogonal; and who owns translation without creating a policy cycle.

10. We need user-experience and workflow guidance, not only type-theory review. Analyze realistic experiences for at least: a project/Charter author setting posture; a policy author mapping posture to assurance; an operator or agent launching an evaluation; a validator/dock reporting limited observation capability; a user seeing a gate result at a friendly default density and drilling down; an agent encountering insufficient authority/Resolution and requesting escalation; and a future Substrate host enforcing permitted effects. Identify where users would face confusing vocabulary, late failure, hidden policy selection, unexplained strictness, accidental coupling, or an inability to answer “why did this block and what do I do next?”

11. Classify recommendations into: (a) changes required in one or both existing proposed decisions before approval; (b) matters for a narrowly scoped explanation-projection semantic decision; (c) matters for a distinct Context Resolution/adoption or Handbook-to-Substrate integration decision; (d) implementation-planning concerns that should not enter semantic records; and (e) ideas that should be deferred. For every proposed new concept, say why an existing concept cannot own it. Call out unnecessary abstraction as well as missing abstraction.

12. Provide discriminating scenarios or counterexamples for every material gap. Pay particular attention to: presentation preference accidentally changing a gate; requested Resolution below claim minima; producer ceiling below requested Resolution; posture selecting a lighter requirement set without weakening trust; evidence sufficient for one claim but insufficient for another; incomparable posture vectors; evaluation replay after posture/stack/policy currentness changes; low-density output omitting a decisive adverse fact; drill-down blocked by disclosure/authority; CLI advisory behavior versus host enforcement; and an agent escalating scope, memory, mutation, or validation reach.

Do not reduce the task or replace it with an easier alternative. Preserve the requested scope and project conventions.

Is the current approach the best way forward? Give a direct keep, adjust, or pivot recommendation; tradeoffs; what to keep/change first; inspections; validation; and pitfalls. In addition, return: (1) findings ordered by materiality; (2) a corrected responsibility map spanning Posture, Assurance Policy, Context Resolution, Contracts, Dock, projection, adapters, and Substrate; (3) the minimum additional semantic decisions needed before implementation planning; (4) a user-centered workflow/mental model with plain-language names; (5) proposed invariants and discriminating scenarios; and (6) explicit answers to whether the Resolution-to-gate relationship is intended or defective, whether underpowered runs should be diagnostic or rejected, and whether explanation detail should reuse or remain type-distinct from Context Resolution. Clearly label what follows from supplied repository truth versus your recommendation or inference.
~~~

A short transport message instructed ChatGPT to read the attached prompt and all
eight sources and to perform the requested fresh independent review.

## ChatGPT Pro response

Worked for 5m 11s

I read the review prompt and all eight supplied project sources as a fresh bounded packet. I did not use prior conversations or prior review conclusions.

Edit
Fresh independent HCM Context Resolution / posture / assurance / metric-gate / UX / enforcement review
Executive verdict: ADJUST

The current approach is the right architecture and should not pivot. The two proposed semantic decisions preserve the most important boundaries correctly: posture selects exact assurance requirements; those requirements do not rewrite metric or gate meaning; canonical evidence and evaluation truth remain closed; gate truth remains non-mutating; and enforcement remains downstream.
I would not approve the two records unchanged, however. They need a small but material Context Resolution amendment before approval. The existing product already makes Context Resolution semantically relevant to evidence eligibility and therefore indirectly to gate truth, while the greenfield metric/evaluation model does not yet give that dependency first-class closure. The proposals also need an explicit statement that Context Resolution capability, posture strictness, and explanation density are three different things.

This is an ADJUST, not PIVOT result. I see no P1 architectural defect requiring replacement of the posture → assurance → evidence → metric/gate chain.

The preferred sequence is:

Amend the two existing proposed records to close the Context Resolution seam and underpowered-run semantics.

Approve those two records if the amendments survive review.

Write one narrowly scoped evaluation-explanation projection decision before user-facing gate/result planning.

Write one narrowly scoped Context Resolution evaluation-readiness/adoption decision before orchestration, preflight, and escalation behavior is planned.

Write a Handbook↔Substrate enforcement decision only when actual host enforcement enters scope.

Do not create “posture resolution levels,” “gate resolution levels,” a universal assurance score, or a generic resolution-aware rendering framework.

1. Repository truth established by the supplied sources

These are source-derived facts, not my recommendations.

Context Resolution is already a real semantic owner

The Engine currently owns the Context Resolution domain, while Contracts owns evidence/verdict/gate semantics. The durable architecture explicitly keeps owner decisions inward of SDK and transport projection.
The shipped Context Resolution stack has four named presets—strategic, coordination, execution, and operation—and six separately ranked dimensions:

scope_horizon

detail_resolution

temporal_horizon

authority_horizon

memory_horizon

validation_horizon

Each dimension has its own ordered domain; there is no one scalar Resolution number.

Resolution already affects evidence and therefore gate truth

Current HCM-5.2 binds a requested six-dimensional Resolution envelope into the evaluation run. Contracts computes effective Resolution dimension-by-dimension from the evaluation request, producer ceiling where present, and actual observation envelope, using the registered Engine stack/ranks. A value below a claim minimum makes that evidence ineligible and visible in accounting.
That means a gate can legitimately fail to obtain a positive result because an observation lacked enough Resolution for a required claim. This is not a second gate or a competing assurance system. It is an evidence-eligibility consequence owned by Contracts using Engine-owned Resolution meaning.

Production projection adoption is narrow, not universal

HCM-4.6 has one production projection path in which projection is bounded by exact Resolution, disclosure policy, vocabulary, currentness, omissions, lossiness, provenance, and rechecks. That is a useful precedent, but it does not establish a universal rendering service through which every Charter, gate explanation, CLI response, or product surface passes.

Current gate semantics are not the proposed greenfield semantics

The implemented HCM-5.2 gate currently derives closed claim verdicts and ultimately exposes only passed or blocked, with hard/required and integrity conditions preceding exact weighted scoring.

The new metric/gate proposal intentionally defines richer future semantics: pass, fail, indeterminate, and not_applicable, while context-construction failures refuse before semantic truth.
That difference is acceptable because both proposed records explicitly identify themselves as proposed semantic decisions rather than implementation authority.

Task-wide host enforcement is not current repository truth

Phase 6 still records bridge/publication/external adoption as unresolved or external-unknown. No supplied source establishes current Substrate enforcement of Handbook Context Resolution.

2. Findings ordered by materiality
P1 Critical

None.

I found no reason to replace the fundamental posture → assurance → evaluation → gate → downstream enforcement decomposition.

P2 Required — 1. Context Resolution is missing from the proposed evaluation closure as an explicit semantic input

This is the largest gap.

The proposed metric record says an evaluation context fixes contract revision, subject snapshot, evaluation time, evidence-set identity, claim-evaluator versions, metric identities/configurations, Evidence Evaluation Policy bindings, and gate policy. When posture participates it additionally fixes the resolved posture/assurance provenance.

But current product semantics already make exact Context Resolution stack meaning and requested/effective Resolution relevant to whether an evidence record can establish a claim. The proposal's list of independent dimensions includes applicability, sufficiency, currentness, consistency, uncertainty, criticality, etc., but does not explicitly name Resolution adequacy.

That omission creates several possible future defects:

the same apparent evaluation context could be replayed against different Resolution stack semantics;

a requested Resolution change could fail to change evaluation identity;

Contracts could accidentally grow its own interpretation of Resolution ordering;

Resolution deficiency could be misclassified as generic sufficiency or missing evidence;

assurance policy could begin treating operational reach as assurance strictness.

Required adjustment

The metric/evaluation record should explicitly add a Context Resolution adequacy dimension or equivalent invariant.

It should state that:

the evaluation context binds the exact Engine Context Resolution stack identity/version/fingerprint and the requested evaluation Resolution envelope;

source/evidence accounting binds applicable producer ceiling, actual observation envelope, and effective Resolution;

exact claim-level Resolution requirements/minima are bound through the selected contract, claim, or compatible evidence-evaluation policy;

Engine alone owns the value domains, ranking, meet/minimum operation, and comparison semantics;

Contracts owns the consequence of that comparison for evidence eligibility and claim evaluation;

Resolution adequacy is categorical/structured, never averaged, weighted, or collapsed into a universal “quality” number.

This is not a new Resolution system. It is the existing Resolution system being given an explicit place in the new evaluation model.

Discriminating counterexample

Two otherwise identical evaluations bind the same evidence bytes and same contract, but one uses stack fingerprint R1 and another uses changed stack semantics R2.

If Resolution participates in evidence eligibility, those cannot be the same closed evaluation context even if the six textual coordinate names happen to match.

P2 Required — 2. The proposals need an explicit semantic answer for a valid-but-underpowered requested Resolution

The review packet reports that current run preparation does not appear to preflight the requested evaluation Resolution against all known claim minima. The supplied HCM-5.2 authority likewise specifies late evidence-eligibility behavior but does not define such a preflight.

The new proposals draw an important distinction:

malformed or incomplete context construction → refusal before semantic truth

valid context whose evidence cannot establish an answer → indeterminate/not-computable, depending on layer.

A requested Resolution that is fully valid under the Engine stack but too low to meet a claim's minimum is not inherently a context-construction defect.

Recommendation

Keep such a run semantically valid for diagnostic evaluation.

Do not redefine “requested Resolution below claim minimum” as an automatic closure refusal. Doing that would destroy useful information: sometimes the exact answer you need is “this evaluation could not establish claim B because the run only had X reach and B requires Y.”

At the workflow layer, however, a decision-seeking evaluation should preflight known limitations before spending work on docks or validators.

Recommended behavior:

If all known applicable gate-relevant claims can in principle be observed under the requested Resolution, proceed.

If a known applicable required claim cannot possibly meet its Resolution requirement, show the gap before execution.

Default workflow may hold and offer either:

request more reach / choose a more capable producer; or

explicitly run the diagnostic evaluation anyway.

That preflight outcome is readiness, not gate truth.

If the user runs diagnostically, the eventual canonical evaluation follows the same semantics as any other evaluation.

I would not introduce separate canonical “diagnostic gate” and “real gate” algorithms. That is unnecessary abstraction. The distinction belongs primarily to orchestration intent and pre-execution workflow.

Discriminating counterexample

Claim A requires detail_resolution = summary; claim B requires full. The run requests normal.

A valid run should be able to establish A while reporting that B cannot be established at this Resolution. The system must not silently lower B's requirement and must not call the entire evaluation context malformed merely because the requested capability was insufficient.

P2 Required — 3. Posture, assurance requirements, and current Context Resolution must be explicitly orthogonal

The posture proposal correctly makes posture a nine-dimensional scoped vector and says Assurance Policy selects exact requirements before evidence and gate evaluation.
What it does not yet say explicitly enough is:

The current run's Context Resolution must not be used to choose a lighter Assurance Policy.

That prohibition matters.

Otherwise:

agent has low observational reach
    -> selector sees low Resolution
    -> selector chooses easier assurance
    -> run passes

would become possible.

That is exactly backwards. Requirements must not shrink because the current actor cannot satisfy them.

Correct relationship

The causal direction should be:

Posture + exact Posture Scope
    -> Resolved Assurance Requirements
        -> exact claims / policies / metrics / gate
        -> exact Resolution requirements carried by those claims/policies, if any

separately:

authorized Context Resolution for this run
    + producer capability
    + actual observation
        -> effective Resolution
        -> Contracts checks whether the already-selected requirements can be satisfied

Assurance Policy may select exact Resolution requirements in the limited sense that it can select:

a contract/claim whose semantics include a Resolution minimum; or

an exact compatible evidence-eligibility policy carrying such a minimum.

But it must not:

invent Engine rank meanings;

modify the Engine stack;

select the actual Resolution granted to a run;

silently elevate authority or memory reach;

reselect easier requirements because the current run has low capability.

The exact selected requirement belongs in Resolved Assurance Requirements transitively. The actual run envelope does not; it is an evaluation-run input.

Important monotonicity caveat

Do not treat Engine Resolution rank as an assurance-strength scale.

For example, authority_horizon = program_policy represents broader operational authority than read_only; that does not mean it is intrinsically “more assured.”

The posture policy's existing requirement-strength relation must remain its own declared semantic relation.

Discriminating counterexample

A hardened security posture selects a claim requiring deep validation. The current agent has only read-only/local observational reach.

Correct result: the requirement stays hard; the workflow may ask for escalation or a capable validator.

Incorrect result: “Hardened requires more assurance, therefore automatically grant the agent broader write authority.”

Posture may require proof. It does not grant privilege.

P2 Required — 4. Explanation density must be a type-distinct post-evaluation projection

The shipped stack already contains detail_resolution = identifier_only | summary | normal | full.

Those labels are tempting to reuse for a friendly gate UI.

Do not.

Current detail_resolution participates in the Context Resolution capability model used by evidence and execution. Changing it can ultimately change whether evidence is eligible. Therefore a user preference such as “show me the simple version” must not be represented by changing that value.

Recommended new semantic decision

Create one narrowly scoped Evaluation Explanation Projection decision.

Its input should be:

one exact canonical evaluation/gate result;

its canonical explanation/accounting;

a presentation-density preference with its own type;

exact disclosure/authorization constraints;

optionally the exact viewer/request context needed for disclosure.

Its output is a separately fingerprinted projection of the same canonical truth.

Use deliberately different user concepts and preferably different internal labels. For example:

Quick answer

Why

Audit trail

Not summary / normal / full, and not “Resolution levels.”

Mandatory floor for every density

Even the lowest-density view must preserve:

semantic gate decision;

if no semantic gate exists, the fact that evaluation was refused;

decisive failed/indeterminate conditions;

material unknowns, stale inputs, or refusals;

selected posture/assurance basis when it materially explains strictness;

any applied exception;

the next actionable remediation or escalation category;

an exact reference to the underlying canonical result;

whether additional detail exists but disclosure prevents showing it.

A lower-density view may omit non-decisive supporting detail. It may not omit the fact that changed the answer.

The detailed view must drill into the same canonical result, not re-run evaluation.

HCM-4.6 already supplies a good design precedent: projection may be lossy, but omissions, lossiness, provenance, disclosure, and rechecks remain explicit.

Discriminating counterexample

Canonical gate result is fail because one security claim is unsatisfied while the aggregate metric is high.

A “Quick answer” projection showing only “Score 99/100” is invalid, even if the omitted claim is visible in the audit view.

The direct decisive adverse fact must survive every allowed density.

P2 Required — 5. Add explicit vocabulary non-equivalence rules

There are now several dangerous collisions:

Existing concept	Must not be read as
Posture Level 1–5	Context Resolution active level/preset
Posture Scope	Context Resolution scope_horizon
Context detail_resolution	explanation density
Context validation_horizon	assurance strictness
Assurance requirements	current agent capability
Semantic gate indeterminate	configuration refusal
Enforcement block/hold	semantic gate fail

The posture proposal already correctly warns that Project Classification and posture are distinct and that posture is not a scalar project-wide class.

Extend that same discipline to Context Resolution.

A short normative subsection in the posture decision is enough. Do not create another ontology document merely for this.

3. P3 Important findings
P3 — 1. Assurance-owner placement has a dependency-cycle hazard

The current durable graph has Contracts depending inward on Engine facilities while Engine does not depend back on Contracts.

HCM-5.2 preserves that direction explicitly.

The posture proposal intentionally defers physical placement of the Assurance Policy authority. That is acceptable at the semantic-decision stage.

But implementation planning must not casually place assurance selection inside Engine if it then needs to resolve Contracts-owned contracts, evidence policies, metric definitions, and gate policies. That could create the prohibited Engine → Contracts back-edge.

The lowest-risk physical placements are:

inside the Contracts domain while keeping “assurance selection” a semantically distinct responsibility; or

in a purpose-specific owner downstream of Engine with a one-way relationship that does not require Engine to depend on it.

This is a planning/placement decision, not a reason to contaminate the semantic record with crate names.

P3 — 2. Refusal, indeterminate gate truth, and downstream enforcement must be visibly separate to users

The new semantic model is stronger than the current implementation precisely because it can distinguish:

evaluation refused — no semantic evaluation was formed safely;

gate fail — the policy established the negative proposition;

gate indeterminate — the policy applies but available truth cannot establish pass/fail;

gate not applicable;

gate pass;

enforcement hold/block — a downstream policy does not permit the workflow to proceed despite preserving the gate result.

The posture proposal already correctly states that the same exact gate truth may produce different downstream dispositions without changing gate semantics.

The UI should never collapse all of these to one red “Blocked” badge.

That would make “why did this block and what do I do next?” almost impossible to answer.

P3 — 3. CLI advice cannot be described as enforcement

The architecture correctly says adapters project owner truth rather than own semantic decisions.

A CLI can therefore:

show the current Context Resolution;

refuse an owner operation whose owner rules reject it;

explain missing authority;

request escalation through an authorized seam.

But passing an envelope into a prompt or printing “read-only” does not constrain an unrelated shell.

Actual task-wide enforcement requires a host/capability boundary.

A future Substrate integration should follow this split:

Handbook semantic authority
    -> exact admitted constraints / disposition
    -> Handbook-to-host translation
    -> Substrate capability grant / restriction
    -> actual tool/process effects
    -> effect/provenance receipt

If Substrate cannot implement a required constraint, the integration must not claim that the constraint is enforced.

The Phase-6 map correctly leaves the external adopter and bridge unresolved today.

4. Corrected responsibility map
Responsibility	Correct owner	Consumes / produces	Must not do
Project/Charter posture authority	Engine-side posture authority and authorized Charter governance	Persisted baseline + nine dimension values; exact scope; Resolved Posture Snapshot; authorized transitions	Select assurance from current agent capability; compute gate truth; self-mutate from gate result
Assurance Policy authority	Semantically distinct assurance-selection owner; physical placement deferred	Resolved Posture Snapshot + exact admitted selector inputs → Resolved Assurance Requirements	Change posture, grant Context Resolution, reinterpret Engine ranks, weaken canonical trust
Context Resolution	Engine	Exact stack/preset semantics, six dimension domains/ranks, requested/authorized envelopes, parent non-expansion, memory/mutation/validation authorization and escalation semantics	Compute posture strictness, choose easier assurance, interpret gate truth, choose UI density
Contracts / claim evaluation	Contracts	Exact assurance requirements + evaluation context + canonical evidence; applies Engine-owned Resolution comparison; establishes applicability/findings/indeterminate state	Invent Resolution order, change requested run authority, reselect posture/assurance
Metric definition/computation	Contracts metric-definition/evaluation responsibility from the proposed record	Compatible claim evaluations → exact named metric result	Treat Resolution, confidence, evidence count, posture, or severity as ambient weight
Gate policy/evaluation	Contracts gate responsibility	Direct claim predicates + exact named metric results → semantic gate decision	Grant privileges, mutate posture/workflow, convert underpowered execution into easier policy
Dock / validator / source provider	Dock/source owner	Source observation, producer capability ceiling where applicable, actual observation Resolution, diagnostics	Upgrade its own Resolution, decide claim applicability, canonicalize evidence, decide gate
Explanation projection	New downstream projection responsibility; physical placement deferred	Exact canonical evaluation/explanation + presentation-density preference + disclosure constraints → bounded user view	Re-evaluate claims, change gate, lower evaluation Resolution, suppress decisive adverse facts
SDK / CLI / Tauri adapters	Adapter owners	Bound acquisition, transport, rendering and error framing	Become posture/assurance/Resolution/gate/enforcement authority
Enforcement Policy	Separate downstream policy owner	Exact gate truth + posture/requirements + declared provenance → operational disposition	Rewrite semantic gate truth or acquire posture-transition authority
Handbook↔Substrate integration	Future explicit integration owner	Exact admitted Handbook constraints/disposition → host-understandable capability request	Invent semantic policy, claim unsupported enforcement, silently relax an untranslatable constraint
Substrate host	Substrate	Applies actual tool, path, credential, network/process/memory capabilities it supports; records actual effects	Reinterpret assurance/gate semantics, claim that advisory prompt text is enforcement

The key dependency rule is simple:

Requirement selection is upstream of current run capability. Current run capability determines whether selected requirements can be established; it never selects weaker requirements.

5. Minimum additional semantic decisions
A. Changes required in the two existing proposals before approval

These should be amendments, not new records:

Add exact Context Resolution stack/request binding to evaluation-context closure.

Add explicit Resolution-adequacy accounting to claim evaluation.

State Engine owns Resolution semantics; Contracts owns evidence consequence.

State Resolved Assurance Requirements may select exact requirements that carry Resolution minima, but actual current Resolution is not a selector input or output.

State a valid but underpowered requested Resolution is not automatically a context-closure defect.

Add the vocabulary non-equivalence rules above.

State that any granted Resolution escalation after run creation requires a fresh run/context; it cannot silently rebind the existing run.

Those changes are enough to make the two semantic decisions internally ready for approval.

B. One narrow Evaluation Explanation Projection decision

Required before implementing the user-facing lower-density gate experience.

It should settle only:

exact canonical source/result identity;

presentation-density type;

mandatory information floor;

lossiness/omission accounting;

disclosure/authorization interaction;

same-result drill-down;

inability to reveal more detail;

projection identity and reproducibility.

It should not define evaluation semantics, posture, assurance selection, or Context Resolution ranks.

C. One distinct Context Resolution Evaluation Readiness and Adoption decision

Required before building broad operator/agent orchestration around these semantics.

It should settle:

how a requested evaluation Resolution is obtained/bound;

comparison against known applicable claim requirements before execution;

what is merely a preflight/readiness result;

behavior when producer ceilings make claims impossible to observe;

diagnostic continuation;

escalation request behavior;

fresh-run requirement after a successful escalation;

which dimensions represent observation/disclosure versus host-authorized effects;

no duplicate ranking/comparison implementation outside Engine.

This record must not define a new Resolution vocabulary.

D. Handbook↔Substrate enforcement decision

Required only before actual Substrate/host enforcement work, not before Contracts/backend semantic planning.

It should define:

translatable capability restrictions;

unsupported-constraint refusal;

actual host grant identities;

effect provenance/receipts;

what remains advisory or semantic-only;

cancellation/currentness/revocation behavior;

no false assertion that a prompt or CLI flag constrained unrestricted tools.

E. Implementation-planning matters that should stay out of semantic records

Keep these for later:

exact Rust types/modules/crates;

whether assurance code lives in Contracts or a new crate;

operation/schema names;

caching;

UI component structure;

migration flags;

database layout;

compatibility bridges;

exact current HCM-5.2 blocked → new fail/indeterminate/refusal mapping;

telemetry;

policy authoring tooling;

actual Substrate transport.

The current two-state implemented gate versus the proposed four-state gate will need particularly careful migration analysis, but that is not a reason to weaken the proposed semantics.

F. Ideas to defer or reject

Do not add now:

a universal project “assurance score”;

a scalar posture strictness score;

a scalar Resolution score;

“gate Resolution”;

“posture Resolution”;

a common low / medium / high detail type reused by evaluation and UI;

a universal projection engine covering every Handbook screen;

automatic posture changes from gate results;

automatic authority escalation to satisfy higher assurance;

cross-context gate composition;

a general marketplace of metric/assurance policies.

These either duplicate existing concepts or collapse deliberately independent dimensions.

6. User-centered mental model

The technical model can be presented much more simply without changing any semantics.

The seven questions
1. Engineering standard

Technical owner: Posture

“How rigorous should this part of the project be?”

For a Charter author, show something like:

Baseline: Production
Security: Hardened
Developer tooling: Product
Everything else: Production

This keeps the convenient 1–5 authoring rubric while making it obvious that the effective result is a vector, not one project score. The proposal already establishes that the five labels are numeric authoring levels and not a universal project strictness scalar.

2. Proof plan

Technical owner: Assurance Policy

“Given that engineering standard, what exactly must we prove?”

Show the selected policy and its major consequences:

required contract;

required claims;

validation/evidence policies;

metric definitions;

gate;

exceptions.

A policy author should be able to preview “for this posture and scope, here is the complete proof plan” before publishing it.

3. Run reach

Technical owner: Context Resolution

“What is this particular operation allowed and able to inspect, remember, validate, or change?”

Avoid calling this “assurance level.”

In friendly UI, the six dimensions could appear as:

Scope reach

Input-detail reach

Time reach

Authority reach

Memory reach

Validation reach

The named strategic/coordination/execution/operation presets can remain advanced conveniences.

4. Observation capability

Technical owner: Dock / validator plus Engine/Contracts comparison

“What could this validator actually observe?”

Example:

Validator can inspect normal detail, but claim security.dependency-integrity requires full detail. That claim cannot be established from this observation.

This is far clearer than a generic “insufficient evidence.”

5. Decision

Technical owner: Contracts gate

“Given the proof plan and what was actually established, what is the semantic answer?”

Friendly meanings:

Pass: required rule was established.

Fail: a required condition was established false.

Could not determine: the rule applies, but the available valid evidence cannot decide it.

Not applicable: the rule genuinely does not apply.

Evaluation refused: no gate decision exists because the required context or authority could not be established safely.

6. View

Technical owner: explanation projection

“How much of the same answer do you want to see?”

UI choices:

Quick answer

Why

Audit trail

Changing this choice must never change questions 1–5.

7. Action

Technical owner: Enforcement Policy and ultimately the actual host

“What may happen next?”

Examples:

proceed;

hold;

block;

ask for approval;

request broader reach;

rerun with a capable validator.

This can differ from semantic gate truth and must be shown as a separate layer.

7. Recommended workflow by actor
Project/Charter author

The author chooses a baseline engineering standard and any explicit dimension overrides.

The UI should not require understanding assurance policies or Context Resolution to author ordinary posture.

A useful preview is:

Production baseline. Security is Hardened. This currently selects Assurance Policy X for scope Y.

Classification-based suggestions remain suggestions; explicit persisted posture remains authoritative, as the proposal already requires.

Assurance-policy author

The author works from:

Posture vector + exact scope
        ↓
candidate Assurance Policy
        ↓
exact proof plan

The authoring tool should preview:

why this policy matches;

dimensions consumed;

selected contract/claims;

exact evidence policies;

any Resolution requirements already carried by claims/policies;

metrics/gate;

exceptions;

monotonicity implications.

For incomparable posture vectors, the tool should show two independent policy resolutions rather than “A is stricter than B.”

Operator or agent launching an evaluation

Before running expensive validators:

Resolve posture and assurance requirements.

Show the exact proof plan.

Bind the requested Context Resolution.

Compare known applicable claim requirements with that requested capability.

Where producer ceilings are known, include them in readiness information.

If a required claim cannot possibly be established:

This run can proceed diagnostically, but cannot currently establish 2 required claims.
Needed: Full input detail and slice-level validation reach.
Options: request more reach, choose Validator B, or run diagnostics anyway.

No policy is relaxed.

Validator or dock with limited capability

A validator states what it can actually provide.

Suppose:

requested: full detail
producer ceiling: normal detail
actual: normal detail

The effective capability is normal.

The dock does not:

claim full;

change the claim minimum;

select an easier assurance policy;

call the claim failed.

Contracts decides the evidence consequence under exact Engine semantics.

User reading a gate result

Default Quick Answer:

Could not determine

One required security claim could not be established because the selected validator only produced Normal detail; the claim requires Full.

Assurance basis: Hardened security posture → secure-release@3.

Next: use the full-capability validator or request the required evaluation reach.

The user can select Why without rerunning anything.

Audit Trail displays the same evaluation's exact claims, evidence accounting, policies, metric calculations, and provenance, subject to disclosure.

Agent encountering insufficient reach

The agent should receive an exact gap, not an automatic privilege grant.

Example:

required validation reach: slice_closeout
current validation reach: unit_closeout

The agent may create an escalation request:

Need slice-level validation reach to establish claim C under Assurance Policy P.

The escalation request is not authority.

If approved, the next operation receives a new Context Resolution envelope. If the prior evaluation run already exists, it remains bound to the old envelope; the agent starts a fresh evaluation.

Future Substrate host

Handbook tells the host what semantic constraints or enforcement disposition apply.

Substrate determines whether it can actually realize them.

For dimensions that map to real host capability, this may include:

writable paths/mounts;

allowed tools;

credentials;

process/network access;

memory access;

possibly environment or execution constraints.

For dimensions such as temporal horizon or presentation detail, enforcement may remain Handbook-side rather than become an OS restriction.

The host must not pretend every Context Resolution dimension is a sandbox primitive.

8. Proposed invariants

I would add or adopt these invariants.

Single Resolution authority. Engine alone defines Context Resolution domains, rank, comparison, and parent/non-expansion semantics.

Requirement/capability separation. Assurance may select a requirement over Resolution; it cannot grant the capability satisfying that requirement.

No capability-driven relaxation. Current Context Resolution, producer ceiling, or actual observation capability cannot select a lighter Assurance Policy.

Exact evaluation binding. A canonical evaluation fixes its exact requested Resolution stack identity and envelope.

No silent rebinding. Posture change, assurance-policy change, Resolution escalation, or stack-semantic change never silently updates an existing run.

Per-evidence accounting. Producer ceiling, actual observation Resolution, effective Resolution, and claim-requirement comparison remain visible where relevant.

Underpowered is not malformed. A valid but insufficient requested Resolution remains distinguishable from an invalid/unresolved evaluation context.

Preflight is not gate truth. A readiness check may stop work before execution but cannot manufacture fail, indeterminate, or pass.

Presentation independence. Presentation-density preference is never an input to posture resolution, assurance selection, evidence admission/evaluation, metrics, or gates.

Decisive-fact preservation. No explanation projection may omit a fact whose absence would materially change how a reasonable reader interprets the decision.

Disclosure transparency. If the canonical explanation contains additional material that the viewer is not authorized to see, the projection says that additional detail is withheld and identifies the applicable disclosure/authority basis at an allowed level.

Escalation is non-authorizing. A request for more scope, authority, memory, detail, or validation reach grants nothing by itself.

Host enforcement must be evidenced. A semantic constraint is not called host-enforced unless the responsible host actually applied it or refused because it could not.

No Resolution-as-assurance ordering. Higher Engine rank does not automatically mean stronger assurance.

9. Discriminating scenario corpus
Scenario	Correct outcome
Presentation preference changes from Quick Answer to Audit Trail.	Same canonical evaluation/gate identity and decision. Only projection identity/content changes.
Requested Resolution below a known required claim minimum.	Evaluation context can remain valid. Readiness warns/holds before expensive execution; diagnostic continuation is allowed if policy/workflow permits. If evidence cannot establish the claim, canonical evaluation reports that limitation; requirements are not lowered.
Producer ceiling below requested Resolution.	Effective Resolution is bounded by the producer ceiling/actual result. Dock cannot claim requested capability. Claims whose minima are met may establish findings; others do not.
Lighter posture selects lighter assurance.	Fewer/less-demanding requirements may legitimately be selected, but canonical admission, authority, provenance, identity, complete accounting, deterministic replay, and fail-closed trust remain unchanged. This matches the proposal's non-relaxable trust rule.
Evidence satisfies claim A but not B because minima differ.	A may establish a finding while B remains indeterminate. There is no global “run Resolution pass/fail.” Gate policy handles the separate claim states.
Incomparable posture vectors.	Neither vector is globally stricter; exact policy resolves each. Already correctly covered by the posture proposal.
Exact evaluation replay after Resolution stack semantics change.	Historical reconstruction uses the exact bound immutable stack semantic identity. A fresh current evaluation under a changed current stack is a new context or refuses under currentness rules; it must not reuse the old evaluation identity.
Posture changes during an evaluation.	Existing evaluation stays bound to the old Resolved Posture Snapshot and requirements. Fresh evaluation required, matching the proposal's current rule.
Resolution escalation succeeds mid-run.	Existing evaluation does not expand. The higher grant is available only to a fresh run/context.
Low-density projection omits the one decisive security failure.	Invalid projection. Density cannot erase decisive adverse truth.
User asks for Audit Trail but disclosure forbids evidence identities.	Gate remains unchanged. Projection shows all permitted explanation plus an explicit withheld-detail marker and authorized escalation path; it does not regenerate the gate.
CLI says operation is read-only but user invokes unrestricted shell separately.	Handbook can truthfully describe its own operation restriction, but must not claim task-wide enforcement. Only a host broker can constrain the unrelated shell.
Host cannot enforce a required network restriction.	Integration refuses/holds or reports unsupported enforcement. It must not silently run while claiming the restriction was applied.
Hardened posture selects stronger proof but current actor has low authority.	Stronger assurance remains selected; actor requests escalation or another execution route. No automatic authority grant.
Same gate truth under two enforcement policies.	Semantic gate remains identical; downstream dispositions may differ, as the posture proposal already allows.
10. Explicit answers to the three central questions
Is the Resolution-to-gate relationship intended or defective?

Intended.

It is architecturally correct for Context Resolution to affect gate truth indirectly through evidence eligibility and claim evaluation.

The correct ownership is:

Engine: what Resolution values mean and how they compare
        ↓
Contracts: whether an observation meets the selected claim requirement
        ↓
claim evaluation
        ↓
metric/gate

That relationship becomes defective only if:

Contracts invents its own Resolution ordering;

Resolution is converted into a generic assurance score;

posture strictness is inferred from current Resolution;

low Resolution causes easier requirements to be selected;

presentation density changes evaluation Resolution.

Current HCM-5.2 already preserves the correct Engine/Contracts ownership split in substance.

Should underpowered runs be diagnostic or rejected?

A valid underpowered evaluation should remain diagnostically legal; decision-seeking workflow should preflight it and normally stop before expensive execution when failure is already inevitable.

That gives you both:

truthful canonical semantics; and

good operator ergonomics.

Do not turn “underpowered” into “malformed” merely for convenience.

Do not create a second diagnostic gate algorithm either.

A readiness/preflight layer can say:

This run cannot currently establish the selected assurance requirements.

If the user elects to proceed, the canonical evaluation tells exactly which claims could and could not be established.

Should explanation detail reuse Context Resolution?

No. It should be type-distinct.

This is the clearest conclusion in the review.

detail_resolution is already a semantic coordinate of execution/evidence capability. A user's preference for a concise explanation must not change that coordinate.

Use a separate presentation concept—e.g. Quick Answer / Why / Audit Trail—and derive every view from the same canonical result.

Context Resolution or another disclosure authority may constrain what may be revealed, but it must not determine which semantic evaluation happened or be reused as the user's density selector.

11. Tradeoffs
Explicit Context Resolution closure adds provenance weight

Evaluation identities and explanations become larger because exact stack/envelope provenance must be retained.

That cost is justified. Resolution already affects truth; hiding the dependency does not remove it, it only makes replay ambiguous.

Diagnostic underpowered evaluations can spend unnecessary work

That is why readiness preflight should exist.

But making underpowered capability structurally invalid would overfit gate workflows and eliminate useful diagnostic behavior.

A separate presentation-density type creates another small policy surface

That is preferable to coupling display preference to a semantic capability dimension whose change can alter evidence eligibility.

Host integration introduces another seam

That seam is unavoidable if actual enforcement is desired. Semantic policy and OS/tool enforcement are genuinely different authorities.

12. Inspections and validation before planning

The attachments support the architectural conclusions above. They do not constitute a new inspection of excluded source code, so the following should be performed when the task moves into bounded planning.

Source inspections

Verify:

every Context Resolution comparison used by Contracts delegates to Engine-owned registered-stack semantics;

exact ownership and serialization of claim Resolution minima;

whether current prepare_evaluation_run_v1 really performs no all-claim Resolution readiness preflight—the review packet reports that absence, but this review does not independently inspect excluded source;

whether any ordinary rendering path currently consumes detail_resolution;

exact call graph for Context Resolution escalation;

whether any current generic transport could be mistaken for host enforcement;

every present HCM-5.2 blocked reason that would need to map to future refusal, fail, or indeterminate.

Semantic validation

Property/scenario checks should establish:

stack fingerprint change cannot replay as the same evaluation;

requested Resolution change cannot replay as the same evaluation;

lower producer ceiling never upgrades to requested capability;

sufficient-for-A/insufficient-for-B remains claim-specific;

current Resolution never changes Assurance Policy selection;

higher posture never implicitly grants authority;

three explanation densities over one evaluation preserve identical decision/ref;

every density includes every decisive blocker or material unknown;

disclosure-denied drill-down preserves the decision and exposes the omission;

escalation request grants nothing;

approved escalation requires a fresh evaluation context;

host inability to enforce a required capability cannot produce an “enforced” receipt.

Posture-policy validation

Keep the proposal's existing property tests for:

relevant-dimension monotonicity;

incomparable vectors;

exact replay;

no classification override;

non-relaxable trust.

Do not add a global scalar test because no global scalar should exist.

13. Final recommendation

ADJUST and continue.

The conceptual architecture is strong enough that a pivot would make it worse. The necessary changes are bounded and mostly clarifying:

Make Context Resolution a first-class closed input to evaluation where it already affects evidence truth.

Preserve the distinction between requirement and current capability.

Keep valid underpowered runs diagnostic, with separate pre-execution readiness.

Explicitly prohibit current Context Resolution from selecting lighter assurance.

Make user-facing explanation density a separate post-evaluation projection type.

Keep actual host enforcement behind a future explicit Handbook↔Substrate boundary.

After those amendments, the two proposed semantic records are a sound basis for human approval and later task-level planning.

The most important mental model to preserve is:

Posture says how rigorous the scope should be. Assurance says what must be proven. Context Resolution says what this run can reach. Evidence says what was actually observed. The gate says what that proves. The view says how much of the same answer to show. Enforcement says what may happen next.

None of those should be allowed to impersonate another.

If useful, I can next turn this review into a very small, bounded amendment checklist for the two proposed decision records—only the exact semantic clauses that should change before re-review.

Sources
Do you like this personality?

## Local reconciliation

The response is advisory. The following reconciliation was performed against
current source and current authority before reporting it to the user.

### Accepted as consistent with repository truth

- The proposed posture -> assurance requirements -> evaluation -> metric/gate ->
  downstream enforcement decomposition should be adjusted rather than replaced.
- Engine is the single semantic owner of Context Resolution value domains,
  ranks, dimensionwise comparison, non-expansion, and related authority
  behavior. Contracts owns the evidence and claim consequence.
- Context Resolution capability, posture/assurance strictness, and user-facing
  explanation density must remain separate.
- The proposed evaluation closure should explicitly bind the exact Resolution
  stack identity and requested evaluation envelope because Resolution already
  affects evidence eligibility.
- Current run capability must never select an easier Assurance Policy or weaken
  previously selected requirements.
- A presentation-density preference must be type-distinct and applied only
  after canonical evaluation. It cannot change posture, requirements, evidence
  eligibility, findings, metrics, or gate truth.
- CLI instructions or an envelope displayed to an agent are not task-wide host
  enforcement. A future Handbook-to-Substrate boundary must distinguish
  semantic constraints from capabilities actually enforced and evidenced by
  the host.
- External Substrate adoption remains external-unknown. No current enforcement
  integration may be inferred.

### Local qualifications and corrections

1. The current implementation already includes
   `requested_resolution`—including its exact stack binding—in the
   `ContractEvaluationRunV1` fingerprint preimage. The gap identified by the
   reviewer is therefore a gap in the proposed greenfield semantic records, not
   a demonstrated collision or replay defect in current HCM-5.2 code.

2. Current Dock behavior is narrower than the review's generalized
   lower-producer-ceiling scenario. `DockProcessRequestV1` requires the run's
   requested Resolution to equal the manifest ceiling, and a completed result
   requires actual Resolution to equal both. Contracts can compute a
   dimensionwise minimum across requested, actual, and an optional producer
   ceiling, but the present first-party Dock path does not exercise a
   requested-greater-than-ceiling completion. Supporting heterogeneous future
   producers would require an explicit design decision rather than being
   described as already implemented.

3. Current evidence admission checks every `observed_claim_id` carried by one
   candidate and refuses the entire candidate if effective Resolution is below
   any one observed claim's minimum. The review's recommended outcome in which
   the same observation establishes claim A while remaining insufficient for
   claim B is therefore available only with separately partitioned candidates,
   or after a future decision changes candidate/admission atomicity. This seam
   must be resolved explicitly in greenfield planning.

4. The current proposed decision records do not normatively mention the typed
   Context Resolution system. The posture record's heading “Resolution and
   selection invariants” uses ordinary posture-resolution language, which
   increases the vocabulary-collision risk. An amendment should name the typed
   Engine domain explicitly and state the non-equivalences.

5. “Valid underpowered runs remain diagnostic, with decision-seeking readiness
   preflight” is a sound product recommendation, not current implemented
   behavior or already-selected authority. Present run preparation does not
   compare the requested envelope against all claim minima. Current evidence
   admission and verdict/gate behavior remain fail-closed, but the future
   boundary among admission refusal, claim indeterminacy, readiness, and
   user-visible remediation still requires a semantic decision.

6. The review recommends treating Resolution adequacy as structured claim
   accounting. Current HCM-5.2 checks it both during candidate admission and
   again during verdict evaluation. The greenfield amendment must decide
   precisely which failures invalidate a candidate, which leave an admitted
   observation ineligible for a particular claim, and which produce
   `indeterminate` claim truth. Calling all three merely “insufficient
   Resolution” would leave the seam underspecified.

### Reconciled next-decision boundary

Before implementation planning, the smallest coherent sequence suggested by the
review and local truth is:

1. amend the two existing candidate decisions to make Context Resolution an
   explicit, exact evaluation input and to prohibit capability-driven assurance
   relaxation;
2. settle valid-underpowered-run, readiness, escalation, and claim-specific
   Resolution-accounting semantics, either in those bounded amendments or in
   one narrowly scoped companion decision;
3. after that, define one separate explanation-projection decision;
4. defer Handbook-to-Substrate enforcement mechanics until a real host/adopter
   boundary is selected.

No repository implementation change is authorized by this consultation.

