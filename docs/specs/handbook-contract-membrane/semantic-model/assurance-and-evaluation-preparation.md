# HCM semantic model — Assurance and evaluation preparation

**Status: APPROVED as semantic input — package approval 2026-09-30; HR-007 integrated 2026-10-04**

Part of the [combined semantic package](README.md). Its approval boundary and
shared greenfield/current-product distinction apply throughout.

This part closes a complete proof plan through scoped posture/assurance selection or direct contract resolution, then follows it through evaluation authorization and readiness. Selection, Engine authorization, capability, and Contracts-domain coordination remain distinct responsibilities.

## Reading map

- [Posture and assurance selection](#posture-assurance)
- [Complete evaluation requirements from two sources](#evaluation-requirements)
- [Evaluation authorization, capability, and readiness](#evaluation-preparation)

<a id="posture-assurance"></a>
## Posture and assurance selection

This chapter owns how an exact resolved posture selects assurance requirements. Evaluation consumes the selected snapshot and requirements without reinterpreting posture.

### Canonical model and vocabulary

The canonical forward path is:

```text
explicit Charter posture + exact scope
  -> Resolved Posture Snapshot P_n
  -> assurance-policy selection
  -> Resolved Assurance Requirements
  -> prospective evaluation purpose and requested observation reach
  -> Engine-owned Evaluation Authorization Binding
  -> separately owned readiness determination
  -> Prospective Evaluation Basis
  -> ordinary collection/admission OR complete-pruning no-collection commitment
  -> sealed collection frontier and canonical evidence-set closure
  -> final Contracts evaluation-context closure
  -> claim evaluations
  -> named metrics
  -> semantic gate decision and canonical explanation
  -> action-specific Operational Reliance Basis, when used operationally
  -> one atomic Operational Reliance Determination
       - Current-Use Eligibility facet
       - fixed Operational Disposition facet
  -> separate Action Intent and Action Authorization Binding or Refusal
  -> Action Attempt, Action Terminal, and state-owner Mutation Receipt only after committed mutation
```

The following terms are normative. Terms in different rows are not synonyms.
In particular, **posture class** is not a canonical term: Project
Classification already names a different concept.

| Term | Fixed meaning and ownership |
| --- | --- |
| **Project Classification** | The separately typed project-origin or change-context category `greenfield`, `brownfield`, `integration`, `modernization`, or `hardening`. Charter authoring may use it to suggest defaults, but the classification itself neither is nor computes posture. |
| **Posture Level** | One value on the Charter's fixed `1-5` rubric, used as a persisted baseline or an explicit value for one Posture Dimension. The current authoring rubric labels levels 1–5 `Exploratory`, `Prototype`, `Product`, `Production`, and `Hardened`; those labels name numeric authoring levels, not posture types or a project-wide scalar. A level has meaning only for its named dimension or for baseline inheritance; it is not a universal project strictness score. |
| **Posture Dimension** | One named, independent axis of engineering posture. The Charter currently has exactly nine ordered dimensions. A project's posture is the vector of their effective levels, not their maximum, minimum, average, or another ambient scalar. |
| **Posture Scope** | The exact bounded subject and applicability boundary for which posture is being resolved, including every identity and currentness binding required to distinguish that subject from another domain, component, operation, or work context. The posture-resolution owner owns this binding and refuses absent, ambiguous, stale, or unsupported scope. |
| **Resolved Posture Snapshot** | The posture-resolution owner's immutable, fingerprinted determination for one exact Posture Scope. It binds the exact Charter identity and revision, persisted baseline, effective value of every Posture Dimension, exact scope, resolution inputs, and currentness basis. It is the only posture form an Assurance Policy may consume. |
| **Assurance Policy** | An immutable, named, versioned semantic rule that declares the exact Posture Scope it supports, the Posture Dimensions it consumes, and how a supported Resolved Posture Snapshot selects assurance requirements. The assurance-policy authority owns the rule; it does not own posture mutation, evidence admission, metric computation, gate truth, or enforcement. |
| **Assurance Policy Selector** | The immutable, named, versioned selection rule that consumes one exact Resolved Posture Snapshot plus only its declared, admitted selection inputs and resolves exactly one applicable Assurance Policy or refuses. Its identity binds its canonical semantic content and currentness, independently of the exact admitted policy-resolution basis and selected policy even when they are packaged together. |
| **Resolved Assurance Requirements** | The immutable, fingerprinted output of applying one exact Assurance Policy to one Resolved Posture Snapshot. Its identity transitively binds the snapshot; exact selector identity, version, semantic-content identity, and currentness; exact admitted policy-resolution basis and currentness; exact selected-policy identity, version, semantic-content identity, and currentness; every admitted exception input; every other declared selection input; and complete candidate accounting and canonical explanation. Its output binds one exact contract revision and its complete contract-rooted expected-claim inventory; any exact claim-set references rooted in that revision; exact compatible Evidence Evaluation Policy bindings; Metric Definitions and complete configurations; one Gate Policy; and admitted exception requirements. Metrics and gates consume this result; they do not derive it. |
| **Engine Context Resolution** | The separately typed Engine-owned authority domain that establishes the bounded operating reach available to an exact objective and context, including its exact stack, dimensions, parent, constraints, mutation rules, and escalation relations. A Posture Level or Resolved Assurance Requirement is neither a Context Resolution coordinate nor authority to exercise one. |
| **Evaluation Authorization Binding** | The exact purpose-bounded Engine-owned result required by the companion metric/gate record for an authorized evaluation context. It is issued from one exact admitted `ContextResolutionEnvelope` and proves the Engine-owned relationship among that envelope, the evaluation purpose, subject or scope, intended use, and requested reach. Assurance selection may supply purpose provenance but neither produces nor replaces this binding. |
| **Prospective Evaluation Basis** | The exactly identified pre-observation provenance closure defined by the companion metric/gate record. It binds the evaluation purpose, subject or scope, objective/use, requested reach, Evaluation Authorization Binding, complete Evaluation Requirements with their source and contract permitted use, and readiness/use-intent provenance, but no evidence-set identity or semantic truth. Posture-selected requirements retain Resolved Assurance Requirements; neither source issues or replaces authorization. |
| **Evidence Evaluation Policy** | The immutable, named, versioned Contracts-owned role-specific policy defined by the companion record. Assurance selection may bind an exact policy only to an evidence-evaluation role and compatibility constraint declared by the selected contract, claim, or evidence requirement. It cannot alter canonical-admission trust, the claim proposition or finding scale, Metric Definition meaning, or Gate Policy meaning. |
| **Metric Definition** | The immutable, named, versioned, fully configured algorithm definition owned by the companion record. Assurance selection may choose an exact definition reference; it MUST NOT change the definition's scale, unit, population, algorithm, parameters, missing-input policy, or numeric meaning. |
| **Gate Policy** | The immutable semantic decision rule owned by the companion record. Assurance selection may choose an exact policy reference; gate evaluation remains a pure application of that resolved policy to compatible claim evaluations and metric results. |
| **Enforcement Policy** | An action-domain Operational Reliance Policy for an enforcement, workflow, or posture action question. It consumes the exact historical gate result and its declared target/currentness inputs to produce one atomic Operational Reliance Determination. It cannot alter gate truth, grant action authority, or authorize posture mutation by implication. |
| **Enforcement Disposition** | The Operational Disposition facet of that one Operational Reliance Determination: `allow`, `block`, `hold`, or `no_effect`, mechanically paired with its Current-Use Eligibility facet. It is not an independently issued, current, supersedable, or replayable result. A Disposition Reference identifies the shared Determination and its exact Operational Disposition facet. |
| **Posture Observation** | An immutable, provenance-bound, admitted fact that may inform later posture reasoning, including an exact gate result when a named observation policy permits it. An observation is neither a recommendation nor authority to mutate posture. |
| **Posture Recommendation** | A separately owned, explainable proposal derived from admitted observations and a named recommendation policy. It identifies a proposed transition but cannot apply it, and its production does not imply approval. |
| **Posture Transition** | The separately authorized, atomic Engine-owned mutation from one exact current posture authority state to the next. Before transition intent or authorization, one transition-specific Operational Reliance Determination MUST establish `eligible / allow` for the exact expected posture target, bounded transition, and scope. A congruent Action Intent, separate Action Authorization Binding, exact Action Attempt Identity, commit-time checks, Action Terminal, and Engine state-owner Mutation Receipt then govern the mutation. Neither an Assurance Policy, gate, observation, recommendation, nor reliance disposition performs or authorizes it. |

### Current product facts and semantic gap

These facts constrain truthful planning but do not make this proposal
implemented:

- The Charter schema fixes `rubric_scale` to `1-5`, bounds the persisted
  baseline to `1..=5`, and permits an optional `1..=5` override for each of the
  exact ordered dimensions `speed_vs_quality`, `type_safety_static_analysis`,
  `testing_rigor`, `scalability_performance`, `reliability_operability`,
  `security_privacy`, `observability`, `dx_tooling_automation`, and
  `ux_polish_api_usability`. Project Classification is a separately typed field
  with the five values named above. The current authoring rubric labels numeric
  levels 1–5 `Exploratory`, `Prototype`, `Product`, `Production`, and
  `Hardened`; these labels do not replace the effective dimension vector.
- The private `ProjectPostureKernel` resolves each effective dimension from the
  persisted baseline and optional level override. It fingerprints the Charter
  authority and resolved vector, but it does not structurally resolve the
  Charter's free-form `domain_overrides` or `default_stance` text into an exact
  Posture Scope or executable policy.
- Broader `PostureEvaluationPolicy` behavior and multi-source observation,
  recommendation, acknowledgement, and hysteresis behavior are absent. The
  shipped profile accepts `posture_evaluation_policy: null`.
- An atomic Engine-owned posture transition exists. It changes one fixed
  dimension leaf through exact authority, approval, compare-and-swap,
  persistence, recovery, and replay checks.
- Existing gate and local-closeout results remain pure derived truth. They do
  not carry posture policy or mutation authority.
- Typed Engine Context Resolution envelopes already bind objectives, exact
  stacks and dimensions, parents, constraints, mutation rules, escalation
  triggers, and private authority admission/currentness. Current Contracts run
  identity binds requested coordinates but no purpose-bounded Engine Evaluation
  Authorization Binding. This proposal therefore selects a semantic seam, not
  a currently implemented public object.

Therefore no current free-form Charter text, classification heuristic, gate
result, or null profile field may be treated as an implemented assurance-policy
mapping.

### Posture resolution and assurance selection invariants

#### Classification does not control posture

- Project Classification MAY inform authoring-time defaults and suggestions.
- A suggestion MUST remain visibly provisional until persisted as explicit
  posture authority through the separately authorized Charter process.
- Persisted explicit posture controls resolution. Classification MUST NOT
  silently override, raise, lower, or relax a persisted baseline or dimension.
- Reclassification alone MUST NOT mutate posture or make an existing explicit
  posture disappear.

#### Posture is a scoped dimension vector

- A Resolved Posture Snapshot MUST retain every effective Posture Dimension.
  It MUST NOT collapse the vector to a maximum, minimum, average, label, or
  universal strictness scalar.
- Each Assurance Policy MUST declare exactly which dimensions it consumes and
  the exact scope for which it can consume them. An undeclared dimension MUST
  NOT be interpreted implicitly.
- Scope resolution MUST be exact and MUST fail closed when any required
  binding is absent, ambiguous, stale, conflicting, or unsupported.
- The Charter's current free-form `domain_overrides` and `default_stance` are
  descriptive authoring inputs, not executable selectors or scope bindings.
  Current free-form `allowed_shortcuts` and `red_lines` likewise MUST NOT
  execute as exceptions or change resolved requirements without the admitted
  typed exception authority defined below.
- A Resolved Posture Snapshot MUST be immutable and fingerprinted. Its identity
  MUST bind the exact Charter identity and revision, persisted baseline,
  effective dimension vector, exact Posture Scope, and the inputs needed to
  establish currentness and deterministic replay.

The posture-resolution and assurance-selection rules below govern the
`posture_selected` requirement source. They are not a universal prerequisite
for the separately defined [direct-contract source](#evaluation-requirements).
Both sources close complete requirements before capability comparison.

#### Assurance selection precedes evaluation

- Assurance-policy selection MUST occur before evidence selection, claim
  evaluation, metric computation, or gate evaluation for the run. Selection
  MUST complete before current evaluation authorization, producer capability,
  operating ceilings, actual observation, or readiness are consulted.
- The Assurance Policy Selector MUST consume only the exact Resolved Posture
  Snapshot and other exact, declared, admitted, and currentness-bound selection
  inputs. Project Classification, mutable posture or Charter state, free-form
  Charter prose, an Evaluation Authorization Binding, requested reach, producer
  capability, a Dock or adapter operating ceiling, actual observation,
  readiness, gate results, enforcement dispositions, and unbound ambient state
  MUST NOT become implicit selector inputs.
- The Assurance Policy Selector MUST resolve exactly one named, versioned,
  supported Assurance Policy or refuse. Missing, ambiguous, stale, substituted,
  or unsupported mapping MUST NOT fall back to a default policy.
- The selector MUST bind and completely account for the exact admitted
  policy-resolution basis it consulted, including every candidate or mapping
  considered and that basis's currentness. Changed selector content, resolution
  basis, selected-policy content, currentness, or another declared input MUST
  change the resolved identity or refuse even when the output references would
  otherwise look equivalent.
- The selected Assurance Policy MUST resolve one exact contract revision and
  its complete contract-rooted expected-claim inventory; any exact claim-set
  references rooted in that revision; exact compatible Evidence Evaluation
  Policy bindings; Metric Definitions and complete configurations; one Gate
  Policy; admitted exception requirements; and a canonical explanation.
- Assurance selection MAY bind an Evidence Evaluation Policy only to a role
  declared by the selected contract, claim, or evidence requirement and only
  when its exact semantics satisfy the declared compatibility constraints. It MUST NOT invent a policy
  role, override claim meaning, affect canonical-admission trust, or silently
  fall back. A missing, incompatible, conflicting, substituted, or unaccounted
  binding MUST refuse.
- Resolved Assurance Requirements MUST completely account for selected,
  excluded, inapplicable, ambiguous, and refused candidates. Their identity
  MUST transitively bind the exact snapshot, selector, policy-resolution basis,
  selected policy, admitted exception inputs, all other declared inputs, the
  currentness of each currentness-bearing input, and that complete accounting.
  An omission MUST NOT become an implicit requirement or relaxation.
- Metrics and gates MUST consume the Resolved Assurance Requirements. They
  MUST NOT reread mutable posture, reinterpret levels or free-form text, select
  alternate requirements, or change the meaning of a Metric Definition.
- When posture influences selection, the Prospective Evaluation Basis and the
  final evaluation context, fingerprint, currentness basis, and canonical
  explanation MUST bind the Resolved
  Posture Snapshot fingerprint; exact Assurance Policy Selector identity,
  version, semantic-content identity, and currentness; exact admitted
  policy-resolution basis identity and currentness; exact selected Assurance
  Policy identity, version, semantic-content identity, and currentness; and
  Resolved Assurance Requirements identity. The requirements identity
  transitively binds the admitted exception inputs, all other declared inputs,
  and complete selection accounting. Replay without those exact bindings MUST
  refuse.
- The corresponding final evaluation context is admissible only after its
  canonical evidence-set identity is fixed and as the exact and complete
  realization of that one Resolved Assurance Requirements identity.
  Its subject snapshot MUST satisfy the exact Posture Scope; its contract
  revision, complete expected-claim inventory, Evidence Evaluation Policy
  bindings, Metric Definitions and configurations, and Gate Policy MUST equal
  the selected outputs; and every selected requirement and every metric or gate
  dependency MUST be closed and completely accounted. Addition, omission,
  substitution, wrong-scope binding, incompatible multiplicity, or an
  unresolved dependency MUST refuse before claim, metric, or semantic-gate
  truth rather than become ordinary `not_applicable`, `not_computable`, or
  `indeterminate` truth.
- Exact realization of Resolved Assurance Requirements is necessary but not
  sufficient to authorize observation or form final semantic truth. Before
  observation, one exact Prospective Evaluation Basis MUST bind those
  requirements and one separately issued exact Engine Evaluation Authorization
  Binding congruent with its subject or scope, objective, intended use, exact
  stack, and requested observation reach. After the canonical evidence-set
  identity is fixed, the final evaluation context MUST bind that exact basis
  and evidence set. The requirements identity neither contains nor replaces
  the run's granted authority. Contracts may exact-check final congruence but
  MUST rely on Engine for issuance lineage, envelope admission, request-within-
  envelope, authority admission, currentness or historical validity, and
  supersession.
- Requirements spanning unrelated contract revisions MUST produce separate
  Resolved Assurance Requirements and evaluation contexts. This record does
  not authorize cross-context gate composition.

#### Admission before assurance selection

Assurance Policy Selectors, Assurance Policies, Evidence Evaluation Policies,
Metric Definitions, Gate Policies, Enforcement/Operational Reliance Policies,
posture-observation and recommendation policies, and exception-policy candidates
MUST follow [shared policy admission](objects-and-governance.md#policy-admission) with intrinsic establishment, applicability and required project qualification distinguished under G4.
Assurance selection consumes those exact admitted definitions; it cannot admit
its own inputs or convert organizational rationale into a waiver.

The shared admission-governance lifecycle establishes the exact project
configuration, owner-rule basis, activation cutover, and authority provenance for the qualifications required by applicable project governance. Assurance governance applies its domain constraints under that
basis; Assurance Selection cannot initialize or replace governing admission rules.
A successor configuration neither substitutes definitions into already resolved
requirements nor automatically revokes their inputs. Required currentness and
dependency checks still apply, with fresh owner results when required.

This is the cross-boundary invariant shared with the
[metric/gate record](evidence-and-evaluation.md#metric-gate): **posture may select
which exact assurance requirements govern a run, but it may never alter the
meaning of the selected contracts, evidence policies, metrics, or gate, and no
metric or gate may reinterpret posture.**

#### Assurance and Context Resolution remain orthogonal

- An Assurance Policy and Resolved Assurance Requirements select proof
  obligations. They MUST NOT issue, grant, broaden, infer, or imply Engine
  Context Resolution authority. An observation requirement may state the reach
  needed to establish a claim; it does not grant that reach.
- An Evaluation Authorization Binding, requested reach, producer capability,
  Dock or adapter operating ceiling, actual observed reach, per-claim evidence
  adequacy, viewer disclosure authority, readiness, and the Prospective
  Evaluation Basis remain separate facts. None may select a lighter Assurance
  Policy or remove, weaken, or reinterpret a resolved requirement.
- If available authorization or capability is below a selected requirement,
  the requirement remains unchanged. The separately owned readiness or
  Prospective Evaluation Basis boundary MUST report or refuse the inability to
  attempt the exact proof plan; assurance selection MUST NOT choose an easier
  plan to fit current reach.
- Missing, stale, superseded, mismatched, or otherwise unverifiable Engine
  authorization before observation refuses Prospective Evaluation Basis
  formation and prevents collection. A failed required Engine verification at
  final evaluation-context formation refuses new claim, metric, or gate truth.
  Earlier launch-check failures and any recovery follow the
  [post-commit verification rule](evidence-and-evaluation.md#post-commit-verification)
  while ordinary collection remains open; no recovery may replace fixed inputs,
  bypass an originally required continuity or terminal-state condition, or
  retroactively authorize collection.
  Neither case may become an exception, implicit waiver, lighter policy,
  evidence insufficiency, `not_computable`, `indeterminate`, or
  `not_applicable` result.

#### Trust invariants do not relax

At every numeric Posture Level, including level 1 (`Exploratory` in the current
authoring rubric), the following remain non-relaxable:

- authority and provenance checks;
- Engine-owned Context Resolution admission and non-expansion;
- canonical admission and exact identity;
- complete claim, evidence, metric, condition, and selection accounting;
- deterministic evaluation and replay; and
- fail-closed behavior for absent, ambiguous, stale, incompatible,
  unauthorized, or unsupported inputs.

Posture may select among admitted requirements. It MUST NOT weaken the trust
needed to establish the inputs, identities, policy, or truth of that selection.

#### Exceptions are typed authority

- Exception governance, typed-policy authority, exact record authorization,
  record admission, and selection-time application are distinct semantic acts.
- Canonical Charter governance MAY nominate an authorization policy, required
  approval or issuer classes, process expectations, record location, and
  minimum authoring fields. Those declarations are governance inputs only;
  Charter prose, approver names, process metadata, record location, and minimum
  fields do not themselves constitute an executable exception policy,
  authorize a particular record, or admit an exception for selection.
- A designated exception-policy authority MUST own the exact typed, named,
  versioned policy that defines permitted issuers, scope, effects, record
  requirements, and currentness. Each exact exception record MUST be separately
  authorized under that policy and admitted through exact authority, identity,
  scope, currentness, and policy-conformance checks.
- Free-form shortcut, red-line, stance, domain-override, rationale, or other
  Charter prose MUST NOT execute as an exception.
- Only an admitted, typed, named, versioned exception policy and its exact
  authorized exception record may affect requirement selection.
- Assurance selection MAY apply only the admitted bounded effect. It MUST NOT
  mint, approve, broaden, infer, or self-authorize an exception.
- The selection explanation MUST identify every applied exception and its
  authority, scope, identity, currentness, and effect. A missing or invalid
  exception MUST fail closed rather than become an implicit waiver.

One eventual component may host more than one of these responsibilities, but
Charter governance nomination, typed-policy authority, exact record
authorization, record admission, and selection-time application MUST remain
semantically distinct and independently attributable.

### Bounded monotonicity and vector order

Monotonicity is bounded to one declared policy family and version, one exact
Posture Scope, and fixed non-posture inputs. Within that boundary, increasing a
dimension that the Assurance Policy declares relevant MUST NOT remove or weaken
any resolved requirement. Each policy family MUST make its requirement-strength
relation explicit enough to establish this property without inventing a global
score.

This rule does not create a total ordering over posture vectors. Componentwise
comparison is valid only when every compared dimension is equal or moves in the
same direction. If one relevant dimension increases while another decreases,
the vectors may be incomparable; selection MUST apply the exact policy rather
than collapsing them to a scalar or guessing which vector is stricter.

### Causal order and run binding

The only permitted causal cycle across evaluations is:

```text
P_n Resolved Posture Snapshot
  -> Resolved Assurance Requirements
  -> prospective evaluation purpose and requested reach
  -> Engine Evaluation Authorization Binding
  -> separately owned readiness determination
  -> Prospective Evaluation Basis
  -> ordinary collection/admission OR complete-pruning no-collection commitment
  -> sealed collection frontier and canonical evidence-set closure
  -> final Contracts evaluation-context closure
  -> metrics / gate
  -> semantic gate decision and canonical explanation
  -> optional admitted Posture Observation
  -> separately owned Posture Recommendation
  -> transition-specific Operational Reliance Basis
  -> one Operational Reliance Determination for the exact posture transition
       - Current-Use Eligibility facet
       - fixed Operational Disposition facet
  -> exact Posture Transition Action Intent, only after `eligible / allow`
  -> Action Authorization Binding or Action Authorization Refusal
  -> exact Action Attempt Identity and commit-bound Engine mutation checks
  -> Action Terminal + Engine state-owner Mutation Receipt after committed change
  -> P_{n+1}
```

- A run remains bound to `P_n` and its Resolved Assurance Requirements. A
  concurrent posture transition MUST NOT silently rebind or reselect that run.
  Its exact Evaluation Authorization Binding is separately fixed in the
  Prospective Evaluation Basis and then bound with the exact evidence set in
  the final evaluation context; neither step converts requirements into
  authority.
- A fresh evaluation under `P_{n+1}` MUST perform fresh scope resolution and
  assurance selection, Engine authorization, readiness, and context closure.
  Results under `P_n` and `P_{n+1}` are distinct contexts even when their other
  inputs happen to match.
- Approval of a Context Resolution escalation creates a new Engine
  authorization basis and requires a fresh Evaluation Authorization Binding,
  readiness determination, Prospective Evaluation Basis, and final evaluation
  context. It MUST NOT mutate the Resolved Assurance Requirements,
  retroactively authorize prior evidence, upgrade a prior semantic decision,
  or rewrite its historical explanation.
- A gate MUST NOT mutate posture, reselect the posture governing its own run,
  or emit a transition as part of pass/fail evaluation.
- Gate `pass`, `fail`, `indeterminate`, or `not_applicable` MUST NOT
  automatically raise or lower posture. A permitted gate result may become an
  admitted Posture Observation only through its separately owned policy.
- A Posture Recommendation remains non-mutating. Before any transition intent,
  one transition-specific Operational Reliance Basis and Determination MUST bind
  the exact expected posture target, proposed transition, current semantic time,
  historical results, target/currentness facts, policy, and complete accounting.
  Only `eligible / allow` permits formation of a congruent Action Intent.
- The Engine transition-authorization owner MUST then independently produce an
  Action Authorization Binding or Refusal for that exact Intent. Policy-permitted
  standalone CLI/SDK flows MAY carry advisory authority from a prompt or
  configured agent role as claimed principal, delegation, and declared authority
  provenance. Those claims are not independently authenticated identity,
  verified delegation, or host-enforced permission; ambient OS permissions are
  not fine-grained Handbook credentials. Advisory authority cannot satisfy a
  policy that requires stronger enforcement or waive expected-target,
  currentness, approval, transition-validity, attempt, or commit-time checks. A
  real enforcing host may supply stronger facts. No general Handbook identity,
  credential, authentication, delegation, capability-token, secrets, PKI, RBAC,
  or ABAC platform is implied.

### Operational reliance and enforcement separation

Enforcement is downstream of immutable semantic gate truth. Each admitted
Enforcement Policy is an Operational Reliance Policy for one exact action
question and target. It MAY consume the exact gate decision, Resolved Posture
Snapshot, Resolved Assurance Requirements, and other declared inputs, but it
MUST form one exact Operational Reliance Basis and one atomic Operational
Reliance Determination with paired Current-Use Eligibility and Operational
Disposition facets. It MUST NOT change the gate, replace `indeterminate` with
`pass`, treat either facet as semantic evidence, or acquire action or posture-
transition authority.

Under the action owner's [exact current-use and Engine-verification rule](operational-reliance-and-actions.md#exact-current-use-and-engine-verification-rule),
a present reliance accepting a Historical Semantic Result whose exact closure
binds an Evaluation Authorization Binding MUST resolve that exact Binding and
Engine-confirmed historical validity. The admitted policy MUST select exactly
one mode for that dependency: `historical_validity_only` or `continuing_current`.
Only `continuing_current` requires fresh positive Engine current verification at
the reliance semantic time and any resulting policy-required fresh evaluation.
An upstream refusal without such a Binding does not acquire one for reliance;
its declared input role still follows the owner's exact-result and complete-
accounting rules. Neither mode weakens mandatory current authorization at
collection or final evaluation closure. A later effect always requires a fresh
separate Action Authorization Binding and exact target/currentness checks.

The same gate truth may legitimately yield different atomic Determinations
under different exact policies or action questions. Each Determination preserves
the shared historical gate identity, its own policy/action/target/currentness
closure, both inseparable facets, and a Disposition Reference to its exact
Operational Disposition facet. The canonical gate explanation remains unchanged; operational reliance
adds downstream accounting and does not reevaluate truth or choose an
explanation-density view.

### Responsibility map

| Responsibility | Semantic owner | MUST NOT own or infer |
| --- | --- | --- |
| Charter authoring and governance | Project authority authors and approvers own Project Classification, persisted baseline and dimension values, descriptive guidance, and authorized Charter changes. | Executable scope matching, assurance selection by prose, metric meaning, gate truth, or automatic posture mutation. |
| Posture resolution | The posture-resolution owner resolves exact scope and produces the immutable Resolved Posture Snapshot from current admitted Charter authority. | Classification-based override, assurance-policy invention, evidence interpretation, gate truth, or transition approval. |
| Policy-definition and admission owners | Accept user-, agent-, artifact-, and tool-authored candidates and deterministically establish intrinsic conformance and, when applicable project qualification is required, produce Policy Admission Closure or refusal under G4 against exact typed vocabulary, relevant governance revisions/proposition roles/scopes/dependency modes, finite accounting, required separately authorized approvals/exceptions, and non-relaxable invariants. The distinct Assurance Selection responsibility resolves admitted selectors/policies and produces Resolved Assurance Requirements with complete explanation. | Treating authorship, prompts, agent roles, artifact promotion, bare links, approval prose, or favorable outcomes as admission; demanding every artifact family; turning governance text into runtime evidence/waiver; recursive self-admission; posture mutation; or gate evaluation. |
| Engine Context Resolution authorization | Engine alone admits the exact full envelope, establishes request-within-envelope and non-expansion, issues the purpose-bounded Evaluation Authorization Binding, and verifies its lineage, authority/currentness, historical validity, and supersession. | Assurance-policy selection or relaxation, producer capability or readiness, evidence adequacy, gate truth, explanation density, enforcement, or posture transition. |
| Contracts-domain Evaluation Coordination | The proposed Contracts-domain coordination responsibility compares already resolved requirements, exact Engine authorization, requested reach, and admitted producer/operating facts; owns readiness and Prospective Evaluation Basis formation; and determines whether the exact proof plan can proceed. Concrete crate, module, API, persistence, and transport placement remain deferred. | Granting Engine authority, selecting an easier policy, weakening requirements, establishing evidence or gate truth, or host enforcement by implication. |
| Exception governance, authority, and admission | Charter governance owns non-executable nominations and process declarations. The designated exception-policy authority owns admitted exception semantics, permitted issuers, scope, effects, record requirements, and currentness; the exact record authority authorizes each record; and the exception-admission owner establishes authority, identity, scope, currentness, and policy conformance before selection may apply the bounded effect. | Treating Charter prose, approver names, process metadata, record location, or minimum fields as executable authority; allowing assurance selection to mint, approve, broaden, infer, or self-authorize an exception. |
| Contract, evidence-policy, evidence, and claim evaluation | The respective semantic owners admit exact contract and evidence authority, own compatible role-specific Evidence Evaluation Policy semantics, and establish applicability and claim truth under the resolved requirements. | Rereading posture, inventing or substituting policy bindings, changing the selected requirements or claim meaning, gate disposition, recommendation, or posture mutation. |
| Metric computation and gate evaluation | The owners defined by the companion record purely apply exact Metric Definitions and Gate Policy inside the closed evaluation context. | Assurance reselection, mutable-posture interpretation, enforcement, recommendation, or mutation. |
| Work Progression and other Operational Reliance owners | Apply one admitted action-specific policy to exact historical results, target/currentness observations, and complete accounting; for consumed results whose closure binds an Evaluation Authorization Binding, resolve its historical validity and select `historical_validity_only` or `continuing_current` under the [action owner's rule](operational-reliance-and-actions.md#exact-current-use-and-engine-verification-rule); and produce one atomic Determination with paired eligibility/disposition facets or a Reliance Refusal. Work Progression is the proposed semantic owner for local-closeout and parent eligibility. | Rewriting gate truth or canonical explanation, issuing action authority, mutating state, or silently selecting work hierarchy/state policy or physical placement. |
| Posture observation | A separately owned admission boundary establishes immutable, provenance-bound observations from permitted sources. | Recommendation, reliance, approval, or mutation merely because an observation exists. |
| Posture recommendation | A separately owned recommendation policy derives an explainable proposal from admitted observations. | Applying, approving, forming transition eligibility by itself, or implying a transition. |
| Posture-transition reliance owner | Forms the exact transition-specific Operational Reliance Basis and one atomic Determination before transition intent, with exact expected posture target, bounded transition, currentness, policy, and accounting. | Treating recommendation as eligibility, issuing action authority, mutating posture, or changing historical gate truth. |
| Transition requester and action-authorization owner | The requester forms one congruent Posture Transition Action Intent only after `eligible / allow`. The authorization owner independently produces an Action Authorization Binding or Refusal with exact advisory, verified, or host-enforced authority provenance and fresh conditions. | Treating reliance, Evaluation Authorization, a prompt, ambient OS permission, or a recommendation as authenticated identity, verified delegation, host enforcement, or a reason to mandate a general Handbook IAM platform. |
| Engine posture-transition executor and state owner | Owns one exact Action Attempt Identity and Action Terminal; enforces the same authorization, expected posture, exact transition, approvals, and declared currentness at the atomic mutation boundary; and alone issues the Mutation Receipt for committed `P_{n+1}`. | Ordinary retry after uncertain commit, broadening the authorized transition, recomputing prior gate truth, or reporting committed posture without the authoritative receipt. |
| SDK, CLI, host, and other transports | Preserve exact owner results and provenance, including policy-permitted prompt- or configured-role-supplied advisory claims; a real enforcing host may supply stronger independently verified facts. | Becoming a semantic owner by transport, fabricating authentication/delegation/enforcement, waiving target/currentness checks, or satisfying enforcement-required policy with advisory claims. |

### Discriminating scenario corpus

Each scenario assumes exact admitted identities and fixed inputs except where
the row names a failure.

| Scenario and inputs | Expected semantic outcome | Invariant proved |
| --- | --- | --- |
| **Mixed dimension levels without collapse.** A snapshot has a lower `speed_vs_quality` level, a higher `security_privacy` level, and distinct values for the other dimensions. A policy declares only those two dimensions relevant. | The snapshot retains all nine effective dimensions. Selection applies the two declared inputs independently; it does not use the vector's max, min, average, baseline label, or an invented project score. | Posture is a vector and policy declares its consumers. |
| **Classification versus explicit posture.** A project is classified `greenfield`, while its persisted `security_privacy` posture is explicit and higher than an authoring suggestion for that classification. | The explicit posture controls. Classification may explain a suggestion but cannot lower, replace, or mutate the persisted value. | Classification and posture are distinct authorities. |
| **PII adapter versus docs-only tool.** One exact scope binds a PII-handling adapter; another binds a docs-only tool. A third request says only “the project area” and could match either. | Each exact scope resolves independently under a supporting policy. The ambiguous request refuses before assurance selection; free-form domain text does not choose for it. | Scope is exact and fails closed. |
| **Lighter assurance selection and trust.** A supported Resolved Posture Snapshot selects a lighter admitted requirement set, but one candidate lacks authority provenance and another leaves expected claims unaccounted. | Selection or evaluation refuses. A lighter requirement set cannot waive authority, provenance, canonical admission, complete accounting, determinism, or fail-closed behavior. | Trust invariants are non-relaxable at every numeric level. |
| **Selected proof plan exceeds current reach.** A higher posture selects a proof plan requiring observation reach that the current Engine authorization, producer capability, or Dock operating ceiling cannot support. | The exact requirements remain selected and unchanged. Readiness or Prospective Evaluation Basis formation reports or refuses the inability to proceed before collection; no lighter Assurance Policy, omitted requirement, or implied waiver is selected. | Current authorization and capability cannot weaken the engineering standard. |
| **Posture changes mid-run.** A run closes on snapshot `P_n`; an authorized transition creates `P_{n+1}` before the gate finishes. | The existing Prospective Evaluation Basis and final context remain bound to `P_n` and its resolved requirements. They do not reselect. A fresh run under `P_{n+1}` resolves a new snapshot, requirements, authorization, basis, and final context; the two contexts cannot be combined. | Runs bind one immutable posture snapshot. |
| **Approved Context Resolution escalation.** The proof plan remains fixed, but Engine approves a broader or otherwise changed envelope after the first authorization binding was issued. | The approval produces a new authorization basis and requires a fresh Evaluation Authorization Binding, readiness determination, Prospective Evaluation Basis, and final evaluation context. The Resolved Assurance Requirements and all prior evidence, result truth, and explanation remain unchanged and bound to their original bases and contexts. | Escalation changes granted reach, not the proof plan or historical truth. |
| **Failed gate feeds posture reasoning.** A gate returns `fail`, and a named observation policy admits that exact result. A recommendation policy proposes raising a relevant dimension. | The gate truth remains `fail`; the admitted observation and recommendation are non-mutating. A transition-specific Operational Reliance Basis and atomic Determination must first establish `eligible / allow` for the exact expected posture and transition. Only a congruent Action Intent, separate Action Authorization Binding, exact attempt, commit-time Engine checks, Terminal, and state-owner Mutation Receipt can create `P_{n+1}`. | Feedback does not create a causal mutation or authority shortcut. |
| **Relevant-dimension monotonicity.** Within the same policy family/version and exact scope, all non-posture inputs and other dimensions are fixed while one declared relevant dimension increases. | The newly resolved requirements preserve or strengthen every prior requirement under the policy's declared strength relation; none is removed or weakened. | Monotonicity is bounded and dimension-relevant. |
| **Incomparable posture vectors.** For the same policy and scope, vector A is higher in security while vector B is higher in operability, and both dimensions are relevant. | Neither vector is declared globally stricter. The exact policy resolves each independently without scalar collapse; any unsupported combination refuses. | A dimension vector has no global total order. |
| **Missing policy mapping.** A valid snapshot and exact scope have no supported Assurance Policy mapping, or two selectors tie without deterministic precedence. | Selection refuses and produces no Resolved Assurance Requirements. It does not choose a default, nearest, lowest, or highest policy. | Missing or ambiguous mapping fails closed. |
| **Different enforcement, same truth.** The same exact semantic gate decision and context are consumed by two different admitted named/versioned Enforcement Policies for distinct action questions or targets. | The exact semantic gate decision and canonical explanation remain unchanged. Each policy closes its own Operational Reliance Basis and atomic Determination; different paired eligibility/disposition facets are possible and each Disposition Reference identifies its shared Determination and exact Operational Disposition facet. Neither result authorizes an effect. | Operational reliance is separate from gate semantics, and eligibility/disposition are inseparable facets rather than independent results. |
| **Free-form shortcut and red line.** Charter text says a shortcut is allowed for a named area and a red line applies to another, but no admitted typed exception policy and record binds the exact scope. A second input is typed but was issued by an unauthorized actor; a third was synthesized or approved by the selector itself. | The prose and both invalid records refuse and do not change requirement selection. Only an exact record that was separately authorized and admitted under the exact exception policy may have its bounded effect applied. | Governance prose, typing alone, and selector self-authorization are not exception authority. |
| **User- or agent-authored policy candidate.** A Decision Record contains organizational rationale and a user or configured agent supplies candidate Assurance Policy text. The submission cites the record by a bare Markdown link, omits exact proposition role/scope/currentness, and has no owner-produced Policy Admission Closure. | The candidate remains governance input and no executable Assurance Policy exists. The semantic owner must deterministically admit or refuse the exact candidate against typed vocabulary, finite candidate accounting, relevant exact governance revisions and dependency modes, required separately authorized approvals/exceptions, and non-relaxable invariants. | Authorship and organizational judgment are permitted; artifact promotion, links, prompts, and agent roles do not perform policy admission or become runtime evidence/waivers. |
| **Advisory CLI posture-transition request.** After an exact transition-specific Determination yields `eligible / allow`, a standalone CLI request carries a prompt-supplied claimed principal and delegation under an action-authorization policy that permits advisory authority. A comparison policy requires host-enforced authority. | Under the advisory policy, the authorization owner may issue a Binding that truthfully records the claims and advisory basis, but it does not assert authenticated identity, verified delegation, or host enforcement; all expected-target, currentness, approval, attempt, and commit checks still apply. Under the enforcing policy, the same advisory claims cannot satisfy authorization and the request refuses absent the required stronger facts. | Advisory authority may participate in the full action chain without fabricating verified facts, waiving safeguards, or creating a general Handbook IAM mandate. |
| **Deterministic requirement replay.** The exact Charter identity/revision, scope, snapshot fingerprint, selector identity/version/content/currentness, admitted policy-resolution basis/currentness, selected-policy identity/version/content/currentness, admitted exception inputs, and other declared inputs are replayed. A comparison case uses a different policy or resolution basis that yields apparently equivalent requirement references. | Exact replay produces the identical Resolved Assurance Requirements identity, references, accounting, and explanation. The comparison case has a distinct provenance-bound requirements identity or refuses; equivalent-looking references do not collapse different authority. Any changed or missing binding likewise changes identity or refuses. | Resolved requirements are exact, transitively closed, and reproducible. |
| **Exact resolved-requirements realization.** One Resolved Assurance Requirements identity for scope `S` selects contract-rooted expected-claim inventory `C`, exact Evidence Evaluation Policy bindings `E`, configured Metric Definitions `M`, and Gate Policy `G`. Before observation, the valid path forms one exact Prospective Evaluation Basis that binds those requirements and a separately issued congruent Evaluation Authorization Binding. After admission fixes the canonical evidence-set identity, the final context binds that evidence set, basis, and exactly `C/E/M/G`. Comparison cases alter either closure or omit, substitute, or transfer the Engine binding or basis. | The exact valid two-stage closure may evaluate reproducibly. Every comparison refuses at the applicable boundary before claim, metric, or semantic-gate truth; none falls back, reselects assurance, or becomes ordinary `not_applicable`, `not_computable`, or `indeterminate`. | Exact pre-observation provenance and later evidence-bound closure are both necessary and neither substitutes for the other. |

### Settled versus deferred

This record settles the canonical terms, semantic ownership, classification and
posture separation, vector and scope model, snapshot closure, fail-closed
selection, Evidence Evaluation Policy selection compatibility, the
one-contract-rooted requirements-to-evaluation conformance rule, evaluation
provenance, assurance/Context Resolution orthogonality, prohibition on
capability-driven relaxation, separate exact Engine authorization conformance,
the pre-observation Prospective Evaluation Basis and later evidence-bound
evaluation-context ordering, fresh-context escalation, non-relaxable trust
invariants, bounded monotonicity, forward-only causal ordering, transition-
specific Operational Reliance before posture-transition intent, the atomic
eligibility/disposition rule, universal historical Binding validity plus the
explicit `historical_validity_only` / `continuing_current` reliance modes,
separate action authorization and mutation receipts, advisory-authority truth
constraints without a general IAM mandate, deterministic Policy Admission
Closure, operational-reliance separation, typed-exception boundary, and scenario
outcomes above.

Deliberately deferred to later human-approved task-level authority are concrete
policy mappings and requirement sets; thresholds; schema shapes; Rust types or
APIs; storage and registry mechanics; expression languages; migrations;
feature flags; slice or packet decomposition; rollout; and implementation
sequence. This record also does not select concrete policy candidates or
mappings, a work hierarchy or state model, physical component placement,
observation or recommendation algorithms, acknowledgement or hysteresis model,
UI, publication, or external adopter.

The proposed
[evaluation authorization, capability, and readiness decision](assurance-and-evaluation-preparation.md#evaluation-preparation)
owns evaluation-purpose formation, Engine authorization issuance, the Contracts-
domain Evaluation Coordination semantic responsibility, Prospective Evaluation
Basis lifecycle, currentness checkpoints, readiness, capability/ceiling
separation, diagnostics, and retry/cache/supersession/escalation semantics. The
proposed [operational-reliance and work-progression decision](operational-reliance-and-actions.md#reliance-actions)
owns the common present-use rule, action chain, and Work Progression semantic
responsibility for local-closeout and parent eligibility. Concrete physical
placement, work hierarchy/state model, policy contents, transition mechanisms,
APIs, schemas, storage, transport, host/Substrate integration, implementation,
and delivery slices remain deferred. These proposals do not silently approve
any such choice or require a general IAM platform.

<a id="evaluation-requirements"></a>
## Complete evaluation requirements from two sources

| Use | What is required, and why |
| --- | --- |
| Author, reference, inspect, validate completeness | Exact owner/content/role truth and the applicable authoring or validation rules. Project admission governance is not inherently required to write a contract, refer to it as data, or determine intrinsic completeness. Such references cannot claim governance qualification. |
| Execute a complete direct contract and report its result | Owner-established contract and compatible full definitions; positive governance applicability resolution under G4; complete direct requirements; compatible intent; independent Engine authorization from approved operating authority; valid capability/readiness; exact collection, evidence, and truth closure. G4 requires a satisfiable supported local path under initial delegation without project admission governance; neither deferral nor the contract supplies execution authority. |
| Establish/admit project governance policies and assign them | First-establishment or prior replacement authority, applicable configuration and owner-rule qualification, exact authorized assignment, scope and currentness. Drafting, validating, admitting, packaging, or discovering a policy does not activate or assign it. |
| Select project posture assurance for a governed obligation | Actual scoped posture and qualified admitted selector/policy/exception inputs, governing relationship and complete selection. A Charter alone does not invent policy mappings; this governed route cannot fall back to independent evaluation when inputs are missing. |
| Use results to discharge governed shipment/completion or authorize its effect | Applicable qualified policies, independently resolved actual target, governing requirements and supported assessment protocol; mandatory assessment qualification/coverage, present reliance, separate action authority and through-commit checks. An earlier direct result may fill declared current roles only through the new owner determination below; it supplies no exemption or authority. |

`admitted` evaluation inputs mean exact owner-established, intrinsically complete
definitions, with applicable project qualification satisfied under G4. Direct
requirements may form while governance is positively deferred; no posture snapshot
or assurance selector is fabricated. The common requirements closure MUST bind
exact governance state/applicability and qualification when applicable, preserving
them through Purpose, basis, final context, result, and explanation under G5.
A damaged or uncertain established basis is not deferral. This route does not
by itself satisfy mandatory governed completion or authorize an action. Current
qualification of an earlier result follows the independently adopted G5 protocol;
it changes neither this original requirements closure nor its occurrence.

The [approved HR-007 amendment](amendments/governance-defaults-and-delegated-operation.md)
was integrated on 2026-10-04. References to G1–G6 below name its approved rules.

Every canonical evaluation MUST consume one complete, exactly identified
**Evaluation Requirements** closure. This is a common semantic role, fulfilled
by exactly one of two sources; it does not mandate another runtime record,
repository, service, or artifact family:

| Requirement-selection source | Responsible owner and closure |
| --- | --- |
| `posture_selected` | Contracts-domain Assurance Selection produces the existing exact Resolved Assurance Requirements from the scoped posture snapshot and admitted selector/policy/exception chain. Resolved Assurance Requirements remains posture-specific. |
| `direct_contract` | A Contracts-domain direct requirement-resolution responsibility closes an explicitly nominated exact admitted, usable contract revision and a complete set of compatible admitted evaluation definitions. It is distinct from admission, readiness, and evaluation. |

The common closure MUST bind and completely account for:

- exact project, subject/snapshot, scope, and owner-established subject bindings;
- exact contract revision/content identity and its owner-established
  [Contract Permitted Use](objects-and-governance.md#contract-permitted-use);
- the complete contract-rooted expected-claim inventory, rooted claim-set
  references, required cases/observation roles, and composition obligations;
- exact applicability and evaluator definitions, every required compatible
  Evidence Evaluation Policy binding, the complete selected Metric Definition
  inventory and configurations, one Gate Policy, and all dependencies;
- exact source owner, source identity, nomination/resolution and selection
  provenance, admitted definitions, required currentness observations, and
  selected/excluded/missing/conflicting accounting; and
- canonical explanation of selection, coverage, and the bounded question.

Required formation and truth-affecting boundary/time rules MUST be fixed at
their existing declared stages, before favorable observations can influence
their selection. An explicitly empty metric inventory is not unresolved metric
selection. A claim-set reference cannot silently remove expected claims.

G5's new current-use qualification has its own independently fixed governing requirements/protocol and finite result accounting; it does not select or revise the original run's requirements or truth-affecting rules after observations.

For direct resolution the caller may nominate permitted contract/rule inputs;
the Contracts owner establishes completeness and compatibility. Missing,
ambiguous, unadmitted, incompatible, or substituted requirements refuse
formation; they do not become defaults, `unknown` readiness, or a smaller plan.
Direct resolution needs no fabricated posture snapshot or assurance selector.
A smaller separately admitted contract is complete relative to its own question,
not automatically to the project's delivery obligation. Admission and governing
assignment remain separate. Neither resolver may admit its own inputs.

Both sources feed the same prospective applicability, Observation Plan, Purpose,
authorization, readiness, collection, evidence closure, final-context, and
claim/metric/gate stages. Evaluation Coordination consumes the requirements; it
does not invent them from available capability. Source identity and provenance,
contract permitted use, and requested Evaluation Use Intent participate in
Purpose content and fingerprint and remain transitively bound through every
result and explanation. Changing source or intent requires a new Purpose and
occurrence even when all selected contract/rule references are equal. Neither
source grants Engine authority, establishes capability, or supplies evidence.

Assurance Selection MAY select an admitted Diagnostic-only contract through a
valid exact mapping; its output preserves that restriction. Selection cannot
implicitly use diagnostic intent or limited capability to choose weaker
requirements. All selection inputs remain declared and admitted. Evaluation
Coordination MUST refuse incompatible contract/use intent before forming a
valid Purpose for Engine authorization. A Diagnostic-only contract requested
with `decision_seeking` is a formation refusal, not a readiness limitation,
`held` assessment, or failed gate. Intent MUST NOT be silently downgraded.

The following combinations assume all other formation and authorization inputs
are valid. Attemptability retains the later readiness table. The final column
applies only to the mandatory [governed-completion assessment](operational-reliance-and-actions.md#governed-completion),
not every permitted downstream use:

Completion-role acceptance is a later owner decision under G5, not formation compatibility or a change to original source/intent. Every diagnostic or incompatible row remains excluded from the mandatory positive role.

| Contract permitted use | Requirements source | Requested intent | Formation compatibility | Mandatory completion assessment |
| --- | --- | --- | --- | --- |
| Diagnostic-only | `direct_contract` | `diagnostic` | Compatible; diagnostic readiness rules apply. | No. |
| Diagnostic-only | `direct_contract` | `decision_seeking` | Refuse before valid Purpose, even if otherwise ready. | No. |
| Diagnostic-only | `posture_selected` | `diagnostic` | Compatible under an exact admitted mapping. | No. |
| Diagnostic-only | `posture_selected` | `decision_seeking` | Refuse; selection cannot widen permitted use. | No. |
| Decision-seeking eligible | `direct_contract` | `diagnostic` | Compatible; diagnostic readiness rules apply. | No. |
| Decision-seeking eligible | `direct_contract` | `decision_seeking` | Compatible; ready may proceed, valid limited/unknown is held. | Potentially, only through an explicitly adopted supported retrospective protocol and new current owner qualification under G5, with actual governing requirements/coverage and all reliance/action checks. No automatic completion. |
| Decision-seeking eligible | `posture_selected` | `diagnostic` | Compatible; diagnostic readiness rules apply. | No. |
| Decision-seeking eligible | `posture_selected` | `decision_seeking` | Compatible; ready may proceed, valid limited/unknown is held. | Potentially, with the actual prospectively resolved governing selection, coverage, adopted protocol and all reliance/action checks under G5. Source alone is insufficient. |

A held or refused project-selected evaluation MUST NOT automatically fall back
to direct evaluation, a narrower contract, or diagnostic intent under the same
Purpose. A deliberately separate admissible development check may proceed but does not by itself discharge the original obligation. Only independently adopted G5 qualification can accept an exact earlier decision-seeking result for a declared role; a held/refused route cannot invent that protocol or replace required prospective provenance. Selecting a Diagnostic-only contract
for a required completion assessment refuses that incompatible route; a valid
diagnostic selection is not a completion exemption.

For example, Diagnostic-only C1 may check response shape and inform C2's authoring.
A later Decision-seeking eligible C3 must be separately admitted and independently
selected for the actual governed obligation. C1/C2 passes never become a C3 pass.
This is illustrative development history, not a mandatory promotion pipeline.

<a id="evaluation-preparation"></a>
## Evaluation authorization, capability, and readiness

This chapter owns the seam between selected requirements and collection: purpose formation, Engine authorization, independent capability facts, readiness, and prospective formation. Current DTOs, crate placement, first-party Dock equality, and advisory command behavior neither constrain nor implement this proposed model.

### Canonical model and vocabulary

The canonical pre-truth chain is:

```text
Evaluation Requirements (`posture_selected` or `direct_contract`)
  -> Observation Plan with exact Producer Selections (zero only under complete pruning)
  -> Evaluation Purpose closing over the exact Observation Plan
  -> Evaluation Authorization Request
  -> Engine envelope admission and non-expansion decision
  -> Evaluation Authorization Binding
  -> Readiness Basis
  -> Readiness Assessment
  -> Attempt Disposition
  -> Prospective Evaluation Basis, when attemptable
  -> ordinary host/producer collection OR Coordination no-collection commitment
  -> sealed collection frontier and terminal admission accounting
  -> exact canonical evidence-set closure (empty for complete pruning)
  -> final Contracts evaluation-context closure
  -> claim / metric / semantic-gate truth
  -> action-specific Operational Reliance Basis and atomic Determination, if used now
  -> separate Action Intent / authorization / attempt / terminal / receipt, if an effect is requested
```

The following terms are normative and non-substitutable:

| Term | Fixed meaning |
| --- | --- |
| **Evaluation Purpose** | An immutable, exactly identified statement formed only after the exact Observation Plan exists. It closes one prospective evaluation's subject or scope, objective, intended evaluation use, complete exact Evaluation Requirements, their selection source, and contract permitted use, and complete Observation Plan identity. It states why that exact plan is requested; it grants no authority and establishes no readiness or truth. |
| **Evaluation Use Intent** | The closed distinction `decision_seeking` or `diagnostic`. It is fixed before Engine authorization and remains exact provenance through any result or downstream use. Changing it creates a new Evaluation Purpose. |
| **Observation Plan** | The immutable, exactly identified pre-authorization closure that assigns the unchanged proof plan's required observation roles to one or more exact Producer Selections, or to zero only through the [complete-pruning route](evidence-and-evaluation.md#complete-pruning). It completely accounts for every required role, its declared multiplicity or composition relation, and each selection's requested reach without scheduling attempts, granting authority, weakening requirements, or establishing evidence or truth. |
| **Producer Selection** | One immutable, uniquely identified entry in an Observation Plan. It binds one declared observation role or exact compatible role set to one producer kind/configuration, any Dock or adapter, Requested Observation Reach, admitted Producer Semantic Capability and operating-ceiling bases, and declared host prerequisites. Selection identities are ordered and unique; attempts, retries, observations, and limitations bind one exact selection and cannot be substituted or merged implicitly. |
| **Evaluation Authorization Request** | The exact request that asks Engine to admit one Evaluation Purpose and the Observation Plan's complete requested-reach closure under one exact `ContextResolutionEnvelope` basis. It is untrusted until Engine admits it. |
| **Evaluation Authorization Binding** | Engine's immutable, purpose-bounded, exactly identified issuance result. It binds the exact Evaluation Purpose, the bound Observation Plan's complete requested-reach closure, underlying admitted envelope, request-within-envelope decision, and opaque Engine authority/admission lineage. Issuance is historical fact; currentness and supersession are separate Engine-owned observations and relations. |
| **Authorization Verification Observation** | Engine's immutable answer at one named checkpoint, verification mode, and semantic time about an exact Evaluation Authorization Binding. Current-use verification returns exactly one of `current`, `superseded`, `not_current`, or `unverifiable`; historical verification returns exactly one of `historically_valid`, `historically_invalid`, or `unverifiable`. It is not a mutable field inside the binding. A historical outcome cannot authorize new collection, no-collection commitment, truth closure, or an operational effect. |
| **Producer Semantic Capability** | An independently admitted, exactly identified statement of the observation dimensions, scales, subjects, and limits a producer kind and configuration can semantically represent under declared conditions. It is neither permission to operate nor proof of what one attempt observed. |
| **Requested Observation Reach** | The exact Resolution-stack-bound reach requested for one Producer Selection within an Evaluation Purpose's Observation Plan. It must be within Engine authorization but is not itself authorization, producer capability, an operating ceiling, actual reach, or a claim minimum. |
| **Dock or Adapter Operating Ceiling** | An admitted, exactly identified operational boundary that a selected Dock or adapter promises not to exceed and may require a request to equal. It is not inherent producer capability, Engine authority, actual observation, or host enforcement. |
| **Host Enforcement Fact** | An exact host-owned observation that named translatable restrictions were actually applied for one attempt, or that launch was refused because they could not be applied. A request, protocol field, acknowledgement, or equality check is not this fact. |
| **Actual Observed Reach** | The exact Resolution-stack-bound reach a producer reports for one completed or partial observation under one exact Producer Selection. It is admitted later with source provenance. It may be lower than requested only when that selection's producer contract permits honest partial completion; it may never exceed the selection request, Engine authorization, or admitted operating ceiling. |
| **Readiness Basis** | The exactly identified, immutable formation-time input closure for complete pre-launch or no-collection pre-commit comparison: Evaluation Purpose, Observation Plan, current Engine authorization verification, Evaluation Requirements with their exact source and contract permitted use, every Producer Selection and its admitted capability/operating-ceiling/host-prerequisite facts, use intent, applicability information available before observation, and every owner-currentness basis used. Later post-commit launch checks remain separate provenance under the committed basis. |
| **Readiness Assessment** | A pure, non-mutating result over one exact Readiness Basis: `ready`, `known_limited`, or `unknown`, with complete per-selection and per-required-role accounting. It predicts whether the complete Observation Plan can answer the unchanged proof plan; it is not evidence, claim truth, metric state, gate truth, permission, enforcement, or a guarantee of success. |
| **Readiness Refusal** | A provenance or formation refusal produced when readiness cannot validly be assessed because any required input is missing, malformed, unadmitted, mismatched, stale, superseded, bound to an unsupported role or dialect, or otherwise unverifiable. It is not a fourth Readiness Assessment state and is never evidence insufficiency or semantic-gate truth. |
| **Attempt Disposition** | The evaluation-coordination result `attemptable`, `held`, or `refused`. After valid Purpose formation establishes contract/use compatibility, a valid Readiness Assessment plus Evaluation Use Intent produces `attemptable` or `held` under the closed table below. A failed authorization or readiness-formation boundary always produces `refused`. It remains advisory until an enforcing host actually honors it. |
| **Prospective Evaluation Basis** | The immutable, exactly identified pre-observation closure formed only for an `attemptable` disposition. It binds the Evaluation Purpose, complete Observation Plan, Evaluation Authorization Binding, the formation-time launch-check or closing no-collection pre-commit verification, Evaluation Requirements with their exact source and contract permitted use, Readiness Basis and Assessment, Attempt Disposition, every Producer Selection and formation-time operating fact, use intent, and required currentness provenance. Later launch checks reference this basis without replacing its inputs. It contains no future frontier or evidence-set identity and establishes no claim, metric, gate, or downstream truth. |
| **Producer Attempt** | One exactly identified launch or collection attempt under one Prospective Evaluation Basis and exactly one Producer Selection. A non-idempotent retry is another attempt for that same selection even if it uses the same basis. |
| **Operational reliance** | A separately owned present-tense use of a closed result under one action-specific Operational Reliance Basis to produce one atomic Operational Reliance Determination with paired Current-Use Eligibility and Operational Disposition facets. Historical display and exact replay are not operational reliance; the Determination grants no action authority or mutation. |

`ready` requires positive proof from valid, admitted, current facts that, for
every required role and exact Producer Selection: the Engine-authorized and
requested reach are congruent; the producer capability can represent the
assigned role and dialect; the operating ceiling permits the attempt; every
required host prerequisite can be met; and the Observation Plan's declared
multiplicity or composition relation can be attempted. Absence of a known
blocker is insufficient. `ready` predicts neither collection success, evidence
sufficiency, nor `pass`.

For the [complete-pruning route](evidence-and-evaluation.md#complete-pruning),
`ready` instead requires positive proof of its complete entry conditions, valid
empty-plan accounting, every required non-producer formation input, and positive
owner verifications at the closing checkpoint. No capability, Dock ceiling,
host-prerequisite, retry, or partial-completion fact is fabricated for a nonexistent
selection. Failed pruning justification refuses empty-plan/readiness formation;
it is not valid `unknown` readiness that diagnostic intent can carry forward.
`attemptable` permits only the no-collection lifecycle transition, not a Producer
Attempt. Contract permitted-use compatibility remains a prior requirement.

`known_limited` means all formation inputs are valid, admitted, and current, but
one or more exact capability, ceiling, host-prerequisite, reach, or plan-
composition facts positively establish that a selected requirement cannot be
attempted by its assigned selection. It is not `fail` or `indeterminate`.
`unknown` means all required formation inputs are valid, admitted, and current,
but their expressly unknown or otherwise insufficient knowledge can prove
neither positive attemptability nor a positive limitation. Missing or invalid
formation input is Readiness Refusal, never `unknown` or `known_limited`.

Both requirements resolvers retain their separately attributable responsibilities
from complete evaluation requirements. For the G5 prospectively governed route,
a request intended to discharge governed completion resolves the actual governing
target and assurance-selection relationship before collection or no-collection
commitment. No future completion Determination or Action Intent is needed.
An arbitrary posture-selected result does not establish governing assignment.
For supported retrospective acceptance, the action domain instead resolves the
current relationship, requirements and protocol before new current qualification;
the earlier evaluation retains its original timing and source. Required prospective
provenance cannot be fabricated. Missing governed inputs cannot select this route
as a fallback.

### Semantic owner placement and identity

A distinct **Evaluation Coordination** responsibility owns Observation Plan and
Producer Selection closure, Evaluation Purpose, Readiness Basis, Readiness
Assessment, Attempt Disposition, Prospective Evaluation Basis formation, and
occurrence commitment/frontier closure, including the complete-pruning route.
The broader Contracts domain is its semantic owner, inward of SDK and
transports, because this responsibility closes Contracts-owned proof
requirements against Engine-owned authorization before the same broader domain
later closes evidence-bound truth. Contracts evidence admission, claim
evaluators, metric computation, and gate evaluation are sibling
subresponsibilities; they consume exact coordination results and MUST NOT
recompute or reinterpret readiness. This settles semantic owner placement.
Concrete crate, module, API, persistence, and transport placement remains
deferred.

This placement is required because the Contracts domain already owns the proof
requirements and final evaluation closure, while the coordination
subresponsibility must consume Engine-owned authorization without acquiring it.
Engine cannot own readiness alone because it does not own contract requirements
or producer semantic adequacy. Assurance selection cannot own it because
present capability must not feed back into requirements. Producers, Docks,
hosts, gates, enforcement, SDK, CLI, and other adapters likewise cannot own the
cross-owner comparison.

Engine alone owns admission of the complete `ContextResolutionEnvelope`,
request non-expansion, issuance and identity of the Evaluation Authorization
Binding, private issuance lineage, authorization verification, historical
validity, not-current reasons, and supersession relations. Evaluation
Coordination and Contracts may exact-check owner-produced identities and
congruence among the subject, scope, purpose, use intent, stack, and requested
reach they own or consume. Engine authorization formation and every later
congruence check MUST refuse when a plan identity, selection, role assignment,
composition/multiplicity relation, or requested reach differs from the exact
Observation Plan bound by the Evaluation Purpose. Coordination and Contracts
MUST NOT reproduce Engine rank, parent, constraint, mutation, authority,
currentness, or escalation decisions.

Every exact formation input participates in identity. Equal coordinate labels
do not collapse different stack fingerprints, subjects, snapshots, purposes,
objectives, uses, parent contexts, constraint closures, authorization lineages,
producer configurations, capability bases, operating ceilings, readiness
times, or currentness bases.

Any changed Observation Plan, Producer Selection, role assignment, declared
plan composition or multiplicity, or Requested Observation Reach requires a
fresh Evaluation Purpose closing over that plan, a fresh Evaluation
Authorization Request and Engine binding, fresh readiness closure, and a fresh
Prospective Evaluation Basis. Neither an older purpose nor an older binding can
authorize a changed plan that happens to target the same subject or coordinates.

Every canonical definition or policy consumed by this chapter MUST follow
[shared policy admission](objects-and-governance.md#policy-admission). In
particular, authored capability, retry, partial-completion, or readiness policy
text cannot become admitted policy through prompt authority or a bare link.

Their owner establishment and any applicable project qualification follow G4; required project qualification follows the shared active-at-decision rule. Configuration
replacement does not rewrite an existing plan, Purpose, readiness result, or
Prospective Evaluation Basis, and grants no Engine evaluation authority. Each
consumer must apply the exact dependency/currentness rules for its bound inputs;
neither automatic re-admission nor automatic continued usability is inferred.

### Observation-plan, evaluation-purpose, and authorization lifecycle

Evaluation Coordination first forms the exact Observation Plan after exact
subject or scope and the unchanged proof plan are available. The plan MUST
completely account for every original observation role and every retained or
omitted relation. Retained roles require one or more exact Producer Selections,
including declared multiplicity/composition. Zero selections are permitted only
under the [complete-pruning conditions](evidence-and-evaluation.md#complete-pruning),
with the exact applicability basis and full original inventory retained. An empty
Plan outside that boundary refuses; it MUST NOT imply scheduling, substitution,
evidence merging, or a dummy producer.
Evaluation Coordination then forms the Evaluation Purpose as a separate exact
closure over that already identified plan and the same complete Evaluation
Requirements. It verifies contract permitted-use/intent compatibility before
supplying a valid Purpose for authorization. The Purpose binds the exact
requirements source and closure. For `posture_selected`, this additionally binds
the exact Resolved Posture Snapshot and Resolved Assurance Requirements lineage;
for `direct_contract`, it binds the Contracts-owned direct-resolution closure
and nomination basis. Each
selection's requested reach is chosen to attempt its assigned part of that
proof plan; no request or producer fact may become an input that selects or
weakens the requirements.

Engine receives the exact Evaluation Authorization Request, admits or
re-admits the complete private envelope and its authority, determines whether
the plan identity and complete requested-reach closure exactly match the plan
bound by the Evaluation Purpose and are within that envelope for the exact
purpose, and either:

- issues one exact Evaluation Authorization Binding;
- returns an exact authorization refusal with decisive owner provenance; or
- returns no owner result when the request cannot be verified.

For complete pruning, the Plan's selection-bound requested-reach population is
explicitly empty. It is not an absent closure, wildcard, fabricated bottom
coordinate, or zero-valued Resolution vector. The complete real envelope,
Profile/stack, subject/scope, objective, use, and authority lineage still close.
Empty request containment establishes only non-expansion of that population;
Engine must independently establish every other authorization proposition. This
Binding permits the exact no-collection lifecycle, not producer collection.
Adding a selection requires the complete new Plan/Purpose/occurrence chain.

Only Engine can issue the binding. A caller, Evaluation Coordination,
Contracts, SDK, transport, producer, Dock, or host may request, carry, compare,
or consume it but MUST NOT mint, broaden, reinterpret, transfer, or reconstruct
it. The public projection MUST expose only stable evaluation-relevant facts and
exact opaque lineage sufficient for congruence and later Engine resolution. It
MUST NOT expose credentials, authenticator material, private witness objects,
repository paths, raw authority capsules, Engine caches or counters, or the
complete private envelope.

At minimum the immutable binding transitively binds:

- its Engine owner identity and exact binding identity;
- the exact Evaluation Purpose and Evaluation Use Intent;
- the exact Observation Plan and complete Producer Selection/request closure;
- subject or subject snapshot and exact scope;
- objective;
- Resolved Profile and Resolution stack identities and semantic fingerprints;
- active level and the Engine-produced admitted bounds relevant to the request;
- the complete selection-bound Requested Observation Reach population, explicitly
  empty only for complete pruning;
- the exact underlying envelope and its root/parent and constraint closure;
- Engine's request-within-envelope conclusion;
- an opaque exact issuance-authority and repository-identity basis;
- issuance semantic time; and
- exact rules naming the checkpoints that require current verification.

Supersession is an immutable Engine-owned relation from an older binding or
request to its successor. It never mutates the older binding. An approved
Context Resolution escalation admits a new envelope and requires a new
Evaluation Authorization Binding; a request for escalation grants nothing.

### Readiness, use intent, and basis formation

Evaluation Coordination creates one Readiness Basis only after Engine has
issued the exact binding and verified it current for the applicable pre-launch
or closing no-collection pre-commit checkpoint. Every
Producer Selection's capability, operating-ceiling, and required host-
prerequisite facts MUST be independently admitted under their named owners and
currentness rules; self-description or a protocol field alone is not an
admitted fact.

One exact Readiness Basis yields exactly one Readiness Assessment or one
Readiness Refusal. The assessment is content-addressed, immutable, and accounts
for every Producer Selection and required observation role. Any change to an
input, role assignment, selection, relevant applicability fact, owner-
currentness basis, or semantic time creates another basis and assessment.

The use-intent rule is closed after valid Purpose formation, including the
contract permitted-use compatibility check. No row below overrides that check:
a Diagnostic-only contract requested with `decision_seeking` already refused
formation, even if its producers could be ready.

| Use intent and readiness | Attempt Disposition | Required behavior |
| --- | --- | --- |
| `decision_seeking` + `ready` | `attemptable` | A Prospective Evaluation Basis may form. The later result is eligible for operational consideration only under separately owned policy and fresh reliance checks. |
| `decision_seeking` + `known_limited` | `held` | No basis forms and no selection launches. Replace the limited Producer Selection through a new exact Observation Plan, choose a different still-authorized request that can realize the unchanged requirements, request Engine escalation, or create a new explicitly diagnostic purpose. |
| `decision_seeking` + `unknown` | `held` | Valid but insufficient capability knowledge is not treated as ready. Obtain an admitted basis that resolves the unknown, form a new exact Observation Plan, or create a new explicitly diagnostic purpose. |
| `diagnostic` + any valid Readiness Assessment | `attemptable` | A diagnostic basis may form, with every known limitation and unknown retained. Evaluation semantics remain unchanged. |
| Any use + Readiness Refusal or non-current/unverifiable authorization | `refused` | No basis forms and no collection or new truth is permitted at that boundary. |

A diagnostic run may produce canonical evidence and exact claim, metric, and
gate truth under the unchanged requirements when all later closure conditions
are met. Evaluators MUST NOT branch on use intent or apply an easier policy.
Diagnostic provenance cannot be relabeled after formation. A diagnostic result
MUST NOT support positive operational reliance unless a separate exact policy
explicitly permits that exact role, direction, and action question within the
mandatory [governed-completion boundary](operational-reliance-and-actions.md#governed-completion).
Diagnostic results cannot fill its required completion-assessment role, including
a fully ready posture-selected diagnostic pass. Permitted remediation remains
distinct. No diagnostic result authorizes mutation by itself.

Prospective Evaluation Basis formation is idempotent for one exact formation
request: the same complete inputs yield the same identity, and the same identity
with different content refuses. A deliberately new evaluation, changed use
intent, replacement authorization, changed readiness-formation input, changed
Observation Plan or Producer Selection, or changed formation-currentness basis
requires a new Prospective Evaluation Basis under the applicable reformation rules.
After commitment, a later checkpoint verification of the unchanged fixed inputs,
including originally permitted bounded operational variation, instead follows
[later launch verification](evidence-and-evaluation.md#post-commit-verification).
It records a new check and launch/refusal outcome without replacing the original
Readiness Basis, Assessment, Attempt Disposition, or Prospective Evaluation Basis.
A change that does require another prospective basis cannot be adopted by the
committed occurrence.

For complete pruning, obtain the closing authorization and readiness-input
verifications first, then form the final Readiness Basis, Assessment, and
Prospective Evaluation Basis over those exact facts. Commit that exact eligible
basis through the [no-collection outcome](evidence-and-evaluation.md#no-collection-commitment).
No second Engine verification is required merely because the basis just formed.
If a genuinely newer readiness-formation observation is required before commitment,
follow the existing bounded pre-commit reformation rule and explicitly commit the
resulting basis; never silently substitute it for the basis named by the request.
Changed Purpose content requires a new occurrence. Earlier unused bases remain
ineligible for both launch and no-collection commitment. Later occurrence-state
checks and the commitment/frontier checkpoint belong to that owner outcome, not
to a retroactive mutation or forward reference inside the prospective basis.
Because this outcome seals collection, it cannot enter the post-commit producer continuation path.

One basis may have zero or more Producer Attempts for each exact Producer
Selection only when that selection's producer policy allows retries. Delivery
replay of the same attempt identity and bytes is idempotent. A new launch,
rerun, timeout recovery, or changed invocation is a distinct attempt bound to
that one selection and has distinct source-observation lineage even when it
references the same still-current basis. Attempts or observations from
different selections or retries are never merged, substituted, or composed
unless a later exact admission/evaluation policy explicitly declares and
accounts for that relation.

### Currentness checkpoints and historical truth

Currentness is an owner observation at a boundary, not an eternal property of
an immutable artifact. Three checkpoints are mandatory:

1. **Before launch, collection, or no-collection commitment, as applicable.**
   Engine MUST verify the exact Evaluation Authorization Binding current and
   unsuperseded. Evaluation Coordination
   MUST establish current permission under the exact committed or prospective
   chain, complete Observation Plan, and applicable capability, operating-ceiling,
   and host-prerequisite rules. Before commitment, fresh formation inputs produce
   the applicable Readiness/Attempt Disposition closure: failed authorization or
   required-input currentness produces `refused`; valid `known_limited` or
   `unknown` facts produce `held` for decision-seeking use. After commitment,
   equivalent blocking conditions are recorded as the later launch's refusal or
   hold under the post-commit rule, without rewriting formation records or
   forming a replacement basis. No decision-seeking launch proceeds while those
   conditions are limited or unknown. Each staggered launch or retry needs its
   applicable exact checks, including plan-wide dependencies; A's permission
   cannot authorize B. The plan cannot be reduced to an unaffected subset. For complete pruning, apply the positive
   empty-route readiness and closing pre-commit basis rules above; invalid inputs
   refuse commitment, and no producer launch or retry is permitted. The exact
   closing verification belongs to the basis committed, not solely to a later
   lifecycle record.
2. **Before final Contracts closure and new truth.** After the exact canonical
   evidence-set identity exists, Engine MUST verify the exact binding under the
   final-closure rule, and Contracts MUST exact-check the evidence set,
   Prospective Evaluation Basis, authorization verification, and every other
   context dependency. Missing, mismatched, not-current, superseded, or
   unverifiable authorization refuses new claim, metric, and gate truth.
3. **Before operational reliance.** The downstream owner MUST resolve the
   exact Evaluation Authorization Binding and obtain the Engine historical-
   validity outcome needed to establish that the closed result was validly
   formed. That historical resolution is universal. The admitted Operational
   Reliance Policy MUST then declare exactly one present-use mode for that
   binding: `historical_validity_only` or `continuing_current`. Only
   `continuing_current` requires a fresh positive Engine current verification at
   the reliance semantic time. `historical_validity_only` makes current
   evaluation-authorization status irrelevant to that Determination while every
   other policy-declared result, target, subject/scope, work-structure, and
   currentness dependency remains exact. A later Action Authorization policy may
   independently require a fresh Engine verification; fresh Action
   Authorization Binding and target checks remain separate and mandatory before
   an attempt. A changed dependency requires a new reliance occurrence or fresh
   evaluation exactly when the admitted policy requires one, never a rewrite of
   historical truth.

A source observation, evidence record, closed evaluation context, claim
evaluation, metric result, gate decision, and canonical explanation remain
immutable historical facts for their exact identities. A later revocation,
supersession, stack change, escalation, or posture transition MUST NOT rewrite
them. A current view may say that a historical result was validly issued under
an earlier basis and is no longer eligible for current reliance.

Historical replay, explanation, and any later Operational Reliance require
Engine resolution of exact issuance and the applicable historical-validity
outcome, not a current-authorization fiction. Merely delivering an already
closed immutable result does not relaunch a producer or reacquire authority.
Re-running evaluation follows the applicable launch/no-collection and final-closure
checkpoints above;
present reliance follows its explicit mode, and any effect follows a fresh,
separate action-authorization and attempt chain.

### Capability, operating, and observation invariants

- Evaluation Requirements state what must be established. They grant no
  capability or authority. Authorization, capability, ceiling, actual reach,
  readiness, diagnostics, retries, escalation, or host limitations MUST NOT
  select an easier policy, remove a requirement, or imply a waiver.
- Engine authorization states what the exact evaluation may request. It does
  not prove producer capability, readiness, actual observation, evidence
  adequacy, gate truth, or host enforcement.
- Producer Semantic Capability states what a producer is admitted to represent
  under exact conditions. It does not grant permission, prove launch, or prove
  actual reach.
- An Observation Plan closes required observation roles over exact Producer
  Selections. It neither schedules attempts nor permits a missing, limited,
  failed, or retried selection to be replaced by another selection implicitly.
- The operating ceiling bounds a Dock or adapter invocation. A producer may
  have broader semantic capability than one operating ceiling, and an operating
  ceiling may be lower than Engine authorization. Neither relationship changes
  the proof plan.
- Host Enforcement Facts exist only when the actual host applies or refuses
  named restrictions. Context Resolution coordinates that lack a faithful host
  primitive remain semantic constraints, not falsely claimed sandbox controls.
- Actual Observed Reach is an observation fact. It MUST be exact and may not be
  clamped upward or downward to match a request, ceiling, or claim minimum.
- Actual reach above Engine authorization, requested reach, or an admitted
  operating ceiling refuses candidate admission. Actual reach below request is
  honest partial collection only under an exact producer policy that permits
  it; later Contracts claim evaluation independently decides per-claim
  eligibility and sufficiency.
- The current first-party Dock rule `request = admitted ceiling = completed
  actual` is one adapter operating contract. It does not prove Engine authority
  or host enforcement and MUST NOT become a universal producer invariant.

Contract permitted use, requirements source, and run intent remain distinct
immutable inputs through these checkpoints. Reusing an exact definition is not
reusing its earlier execution evidence. A new project-selected occurrence needs
its own authorized prospective basis and occurrence-bound collection/closure, or
its own complete-pruning commitment/frontier and empty closure when eligible;
existing evidence-import restrictions remain unchanged.

### Retry, cache, supersession, and reuse rules

| Event | Required semantic behavior |
| --- | --- |
| Exact request replay before launch | Engine issuance may be idempotent only for the complete same authorization request and lineage; Engine still supplies checkpoint verification. |
| Deliberately new evaluation with apparently equal inputs | Close the exact Observation Plan into a new Evaluation Purpose and create a fresh authorization request and binding. Do not silently transfer the earlier purpose or binding. |
| Changed plan, selection, assignment, multiplicity/composition, or requested reach | Form a fresh Observation Plan, Evaluation Purpose, authorization request and Engine binding, readiness closure, and Prospective Evaluation Basis. Reuse or congruence against an older plan refuses. |
| Attempt retry under unchanged basis | Keep the committed basis immutable; require the selection's original unexhausted retry allowance, open collection, and the exact later owner checks and Coordination outcome under the post-commit rule. Create a new selection-bound attempt identity for a real execution; exact delivery replay alone is idempotent. |
| Later check unavailable, negative, or contradictory | Block the affected launch and retain its exact owner outcome/reason. Further verification or resumption needs originally declared recovery/continuity permission; no later success erases the failure or retroactively authorizes activity. A changed fixed input requires the applicable reformation/new occurrence chain. |
| Authorization becomes non-current before launch or no-collection commitment | Refuse that boundary; before commitment the Attempt Disposition is `refused`, while a later launch records its own refusal without replacing the committed disposition. Do not launch or commit the no-collection route. `held` is unavailable for failed authorization. |
| Authorization fails the required final-closure check after collection or no-collection commitment | Preserve exact commitment/frontier and any attempt or observation provenance, but refuse new Contracts truth. Later launch checks do not waive this separate boundary, erase earlier invalid activity, or permit a fresh authorized run to adopt or upgrade old observations. |
| Authorization changes after truth closes | Preserve the closed result. Any present reliance universally resolves the exact Binding and historical validity, then follows its admitted `historical_validity_only` or `continuing_current` mode. Only the latter requires current Engine authorization for the Determination. A later effect still requires fresh separate action authorization and target checks; a fresh evaluation is required only when the selected reliance or authorization policy makes the changed Engine proposition a present dependency. |
| Approved escalation | Engine issues a fresh binding from the newly admitted envelope. Readiness, Prospective Evaluation Basis, attempts, evidence-set identity, and final evaluation context are fresh. Requirements remain unchanged unless separately reselected for an independent reason. |
| Refused escalation | Existing authorization remains unchanged if otherwise current; no requested reach or requirement changes. |
| Superseded request or binding | Follow Engine's exact successor relation. The superseded object remains historical and cannot authorize new collection or new evaluation truth. Present reliance may consume an already closed result only through universal historical Binding resolution/validity and the policy-selected `historical_validity_only` or `continuing_current` mode; the superseded binding never becomes action authority. |
| Partial collection or operational timeout | Record a distinct per-selection attempt terminal and any permitted exact partial observation. Do not fabricate completion, actual reach, evidence sufficiency, plan completion, or gate truth. A retry is another attempt for that selection. |
| Complete-pruning closing facts refreshed before commitment | Under the existing unchanged-input pre-commit rule, form fresh readiness/basis identities and explicitly commit that basis. Earlier unused bases cannot support launch or no-collection commitment. |
| Exact no-collection outcome replay or uncertain delivery | Resolve the same Coordination outcome, basis, frontier, and semantic checkpoint without re-execution or new checkpoint facts. Uncertainty proves neither success nor non-commit; resolve the owner outcome before progression. |
| Readiness cache reuse | Exact cached formation records establish only their original checkpoint. Current launch permission requires applicable fresh owner checks. Before commitment, changed formation facts follow readiness reformation; after commitment, permitted later checks have separate provenance. Changed fixed inputs cannot be hidden as cache refresh or replace the committed basis. |
| Current-result cache reuse | A cached result may supply only exact immutable historical truth. A new present-use decision requires an exact Operational Reliance occurrence, universal historical Binding resolution/validity, the policy-selected `historical_validity_only` or `continuing_current` checks, every other declared current dependency, and complete accounting. A cached Determination cannot authorize an effect; fresh action authorization and target checks remain separate. |
| Historical-result cache reuse | Permit exact immutable replay or explanation only, with exact historical Binding resolution and owner-attributed resolution outcome. Label any present reliance, current Engine status, disclosure limit, and action authority separately. |

No cache can convert historical validity into current authorization, make a
policy omit its required mode, convert evaluation authorization into action
authorization, replace an owner verification, merge or substitute selections,
attempts, or evidence roots, or make a diagnostic result decision-seeking.

### Advisory and enforceable behavior

Later launch verification uses the existing Engine, input owners, and Evaluation
Coordination responsibilities. It requires exact attribution and accounting, not
a new authorization platform, scheduler, or dedicated record store. A positive
prerequisite check is prospective; it does not assert that a host actually
enforced restrictions during the subsequent attempt. Required actual enforcement
still has separately attributable Host Enforcement Facts.

Evaluation Coordination produces semantic owner results. A CLI, skill, or
protocol-only consumer may display `attemptable`, `held`, or `refused`, preserve
the exact reasons, and decline an operation routed through itself. Such behavior
is advisory outside that consumer's own call boundary. Agent instructions and
protocol acknowledgements do not prove that filesystem, network, process,
memory, CPU, tool, or other host restrictions were enforced.

That evaluation boundary does not impose a general Handbook identity,
credential, authentication, delegation, capability-token, secrets, PKI, RBAC,
or ABAC platform. Engine Context Resolution still owns semantic admission and
verification of its Evaluation Authorization Binding. Separately, a standalone
CLI/SDK action path may carry user-prompt or configured-agent-role intent,
claimed principal/delegation, and declared **advisory** action authority when an
exact downstream Action Authorization policy permits that mode. Those claims do
not become Engine evaluation authorization, independently authenticated
identity/delegation, or Host Enforcement Facts; ambient operating-system
permissions are real execution conditions but are not fine-grained Handbook
credentials. Advisory authority cannot satisfy a policy that requires stronger
owner-verified or host-enforced facts, and it cannot waive the action target,
currentness, or commit-time checks fixed by the operational-reliance companion.

An enforcing host or future Substrate adopter may launch only from one exact
current Prospective Evaluation Basis, translate only constraints it actually
supports, apply or refuse those constraints, and emit exact Host Enforcement
Facts and attempt provenance. It MUST NOT infer untranslatable enforcement,
change assurance requirements, broaden Engine authorization, reinterpret gate
truth, or acquire posture-transition authority. This record selects no host,
adopter, constraint translation, sandbox, tool protocol, or enforcement
mechanism.

### Responsibility map

| Responsibility | Semantic owner | MUST NOT own or infer |
| --- | --- | --- |
| Evaluation Requirements | Contracts Assurance Selection closes posture-selected requirements; Contracts direct requirement resolution closes complete nominated-contract requirements. Each preserves exact admitted contract permitted use before capability comparison. | Engine authority, readiness, easier requirements from current capability, evidence truth, or host enforcement. |
| Contracts-domain evaluation coordination | The broader Contracts domain's distinct coordination subresponsibility forms Observation Plan and Producer Selections, closes them into Evaluation Purpose, and forms Readiness Basis and Assessment, Attempt Disposition, and Prospective Evaluation Basis; it owns exact identity, cardinality, role accounting, and per-selection retry/reuse rules for those objects, later launch permission/hold/refusal over exact owner checks and recovery accounting, plus occurrence commitment/frontier closure, including the complete-pruning outcome. | Engine admission or issuance, capability self-attestation, evidence admission or claim/metric/gate truth, operational enforcement, or operational state mutation. |
| Engine Context Resolution | Engine admits complete envelopes, proves request non-expansion, issues Evaluation Authorization Bindings, preserves private lineage, and owns verification, historical validity, currentness, and supersession. | Assurance selection, producer capability, readiness comparison, evidence adequacy, gate truth, host enforcement, or a general Handbook IAM platform. |
| Producer-capability authority | A named independent owner admits versioned Producer Semantic Capability facts and their currentness. | Engine authorization, actual reach, evidence sufficiency, or permission to launch. |
| Dock or adapter | Owns its admitted operating ceiling, request compatibility, invocation protocol, and honest operational terminals. | Engine authority, universal producer capability, host enforcement, canonical evidence, or semantic truth. |
| Enforcing host | Applies or refuses supported runtime restrictions and records exact Host Enforcement Facts and attempt effects. | Resolution semantics, assurance selection, readiness, evidence admission, gate interpretation, or invented enforcement. |
| Producer | Performs one exact attempt and reports source observations and Actual Observed Reach with provenance. | Canonical evidence admission, readiness reclassification, claim adequacy, gate truth, or authority expansion. |
| Contracts-domain evidence and evaluation subresponsibilities | Sibling subresponsibilities admit candidates, fix the evidence-set identity, close the final evaluation context with the exact Prospective Evaluation Basis and Engine verification, and own claim/metric/gate truth under the companion record. | Engine currentness or authorization issuance; producer capability or host enforcement; recomputing or reinterpreting coordination-owned readiness; or requirement relaxation. |
| Work Progression and other Operational Reliance owners | Resolve exact historical results and Binding validity, select the explicit `historical_validity_only` or `continuing_current` mode, close target/currentness/accounting including independently resolved governing-completion requirements, adopted protocol, selection/qualification and complete assessment coverage under G5 where applicable, and produce one atomic Determination with paired eligibility/disposition facets. Work Progression is the proposed semantic owner for local and parent eligibility. | Rewriting historical truth, relabeling diagnostic provenance, treating readiness as a gate, issuing action authority, or mutating state. |
| Action requester and action-authorization owner | The requester owns one exact Action Intent. The authorization owner independently produces an Action Authorization Binding or Refusal with exact advisory, verified, or host-enforced authority provenance and every declared fresh condition. | Treating Evaluation Authorization as action authorization, fabricating authentication/delegation/enforcement, improving reliance eligibility, or mandating a general Handbook IAM implementation. |
| Action executor and authoritative state/effect owner | The executor owns one exact Action Attempt Identity and Terminal. The state/effect owner performs through-commit expected-state and authorization checks and alone issues the Mutation Receipt for a committed change. | Ordinary retry after uncertain commit, broadening the authorized effect, or reporting mutation success without the authoritative receipt. |
| SDK, CLI, Tauri, skills, and other transports | Preserve and project exact owner results, correlation, presentation, interface-local failures, and prompt- or role-supplied advisory-authority claims with their provenance. | Semantic formation, owner verification, capability inference, authentication by assertion, task-wide enforcement, or truth reinterpretation. |

### Discriminating scenario corpus

Each scenario assumes compatible contract permitted use and the exact source
closure unless testing a refusal. It holds the selected requirements fixed and
assumes exact admitted
identities except where the row identifies a failure.

| Scenario and inputs | Expected semantic outcome | Invariant proved |
| --- | --- | --- |
| **Complete pruning.** Every original claim is positively inapplicable and all original observation roles are accounted as omitted. | Form an explicit empty Plan/request population, obtain real Engine authorization, establish positive `ready`, and form the exact closing-check basis. Coordination may commit and seal empty collection; no producer facts or attempts are invented. | Positive proof, not absence of blockers, justifies this route. |
| **New required check before no-collection commitment.** B0 contains V0; required verification V1 is obtained before commitment with all permitted stable inputs unchanged. | Form B1 through the bounded pre-commit refresh rule and explicitly commit B1. B0 remains immutable and unused; no silent substitution or redundant recheck is required merely by forming B1. | The committed basis contains the checks actually used. |
| **Empty plan without proof.** Tools are absent, a claim is unresolved, or a retained shared role needs collection. | Refuse the requested empty Plan; retain the affected requirements. Diagnostic intent and apparent readiness cannot waive entry conditions. | No capability-driven pruning or diagnostic bypass. |
| **Ready multi-producer decision-seeking run.** One exact Observation Plan assigns schema conformance to selection A and runtime side effects to selection B. Valid, admitted, current facts positively prove for every role and selection that authorized/requested reach is congruent, capability represents the role/dialect, the operating ceiling permits the attempt, host prerequisites can be met, and the declared two-selection composition can be attempted. | Readiness is `ready` with complete per-selection/role accounting, disposition is `attemptable`, and one Prospective Evaluation Basis may form. Attempts bind A or B exactly; collection success, evidence sufficiency, pass, and all other semantic truth remain unproved. | Readiness requires positive proof across the complete plan without treating producers as interchangeable or predicting an evaluation outcome. |
| **Known-limited decision-seeking run.** Every formation input is valid, admitted, and current, but selection B's exact operating ceiling positively cannot meet its assigned gate-relevant requirement. | Requirements and assignments remain unchanged; readiness is `known_limited`; disposition is `held`; no basis forms and no selection launches. | A valid limiting fact produces `known_limited` and cannot relax assurance. |
| **Known-limited diagnostic run.** The same request is deliberately re-formed with `diagnostic` intent and Engine issues a congruent binding. | A new purpose, binding, readiness basis, assessment, and diagnostic Prospective Evaluation Basis form. Any later canonical truth retains diagnostic provenance and cannot support positive reliance unless an exact downstream policy permits its role; it can never fill the mandatory governed-completion assessment role. | Diagnostics are exact new provenance, not a weaker gate or relabeling. |
| **Valid but inconclusive capability knowledge.** Every required formation input is valid, admitted, and current, but selection B's admitted capability fact explicitly leaves a necessary semantic limit unknown, so the comparison proves neither adequate nor inadequate reach. | Readiness is `unknown`. Decision-seeking is held; a diagnostic purpose may be attemptable. Unknown never defaults to ready. | Valid insufficient knowledge is distinct from missing or invalid input. |
| **Invalid readiness formation.** A required capability record is missing or stale, or a Producer Selection names a role/dialect outside the admitted capability contract. | Readiness Refusal is produced; there is no Readiness Assessment, basis, launch, evidence insufficiency, or gate truth. | Missing, invalid, and unsupported formation inputs never become `unknown` or `known_limited`. |
| **Coordinates without Engine binding.** Stack and coordinate values are valid, and a Dock accepts them, but no binding exists. | Authorization or basis formation refuses before launch. No readiness assessment, observation, or semantic truth is fabricated. | Structural equality and Dock acceptance are not authority. |
| **Same coordinates, different lineage.** A real binding is reused for another subject, objective, parent, constraint closure, use intent, or same-labelled changed stack. | Engine or congruence checking refuses. A fresh purpose and binding are required. | Authorization is purpose-bound and non-transferable. |
| **Plan changed after purpose formation.** The subject and coordinate labels are unchanged, but selection B, one role assignment, declared multiplicity, or one requested reach differs from the Observation Plan identity bound by the Evaluation Purpose. | Engine authorization formation or later congruence refuses the mismatch. A fresh Observation Plan, Evaluation Purpose, authorization request/binding, readiness closure, and Prospective Evaluation Basis are required. | A purpose closes one exact plan; apparent coordinate equality cannot authorize plan substitution. |
| **Current Dock equality.** Requested reach, manifest ceiling, and reported completed actual reach are equal, but Engine authorization is missing. | The Dock operating contract may be internally coherent, but no valid Prospective Evaluation Basis or canonical evaluation truth can result. | Equality proves neither Engine authority nor host enforcement. |
| **Ceiling above request.** An admitted producer/adapter ceiling is broader than the exact authorized request, and separate valid current facts positively establish every other readiness condition for the complete plan. | Readiness is `ready`; the request remains the operative bound and actual reach may not exceed it. No automatic widening occurs. | A permissive ceiling participates in positive readiness proof but is not permission to widen. |
| **Honest partial observation.** Selection B's producer policy permits partial completion and its attempt reports actual reach below that selection's request after a ready launch. | The exact B-bound attempt and partial observation may proceed to admission if otherwise valid. Contracts decides claim-specific eligibility under the declared plan relation and may later establish indeterminacy; readiness is not rewritten retroactively and A's observations are not merged implicitly. | Actual reach, selection identity, and pre-launch readiness are different facts. |
| **Actual exceeds a bound.** A producer reports reach above request, Engine authorization, or admitted operating ceiling. | Candidate admission refuses; the value is not clamped. No claim, metric, or gate truth is produced from it. | Actual-observation honesty is fail closed. |
| **Authorization superseded before launch.** Engine reports the binding superseded at the launch checkpoint. | Disposition refuses; no selection launches. The immutable binding remains historically issued and retains its successor relation. | Issuance, currentness, and supersession are separate facts. |
| **Authorization fails final verification after collection.** Exact observations exist, but Engine's required final-closure verification reports the binding non-current. | Attempt and observation history remain immutable; final closure refuses new claim, metric, and gate truth. A fresh binding cannot retroactively authorize the old collection. | Pre-launch permission does not bypass the pre-truth checkpoint. |
| **Authorization changes after a pass.** A valid `pass` closed before the binding became non-current. One Operational Reliance Policy declares `historical_validity_only`; another declares `continuing_current`. | The historical `pass` and canonical explanation remain unchanged, and both reliance paths must resolve the exact Binding and historical validity. The first may form its Determination without treating current Engine status as an input when every other dependency closes. The second refuses absent a fresh positive Engine current verification and requires a fresh evaluation when its policy requires one. Any effect under either path still requires a distinct fresh Action Authorization Binding, target verification, attempt, and state-owner receipt. | Historical validity, policy-selected continuing-current reliance, and fresh action authorization are separate propositions. |
| **Approved escalation.** The proof plan is unchanged, and Engine approves broader reach after a prior basis exists. | Engine issues a fresh binding; readiness, basis, attempts, evidence set, and final context are fresh. Prior evidence and results are not upgraded. | Escalation is not in-place expansion. |
| **Per-selection retry after timeout.** Selection B's attempt times out after an exact permitted partial observation; its producer policy permits retry under the still-current same basis, while selection A already completed. | A's attempt remains unchanged. B's partial terminal remains immutable and a new B-bound attempt identity is created; it cannot substitute for A or merge implicitly with either prior attempt. Exact delivery replay alone is idempotent. | Retry is per selection, not plan-wide overwrite or duplicate authority. |
| **Stale readiness cache.** Authorization is still current, but producer capability or operating-ceiling currentness changed after assessment. | Cached formation readiness alone cannot establish current launch permission. Before commitment, changed formation inputs follow the bounded readiness-reformation rule. After commitment, retain the original formation records and committed basis; record the applicable later checks and Coordination permission, hold, or refusal under the post-commit rule. Replacement of a fixed input follows the applicable new-occurrence chain. | Formation identity and later launch permission remain separate; a cache supplies no fresh verification. |
| **Protocol-only restriction.** A CLI passes an exact requested reach and the process acknowledges it, but no host applies a corresponding restriction. | The request and acknowledgement remain advisory/operational facts. No Host Enforcement Fact may claim the restriction was enforced. | Declared intent is not host enforcement. |
| **Host cannot enforce.** A selected attempt requires a translatable restriction the actual host cannot apply. | The host refuses launch or downstream policy holds. It does not silently run, weaken requirements, or claim enforcement. | Only the actual host proves applied restriction. |

### Settled versus deferred

This record settles the canonical vocabulary; complete pre-truth stage order;
the common Evaluation Requirements role with posture-selected and Contracts-owned
direct sources; immutable source and contract permitted-use provenance;
contract/use compatibility before Purpose formation; permitted posture-selected diagnostics without completion substitution and protocol-specific prospective assessment or current qualification under G5; the Contracts-domain Evaluation
Coordination semantic owner placement;
Observation Plan and Producer Selection closure; purpose, authorization,
readiness, disposition, basis, and per-selection attempt identity/cardinality;
the exact `known_limited`/`unknown`/Readiness Refusal boundary; Engine issuance,
projection privacy, verification, historical-validity, currentness, and
supersession responsibilities; complete-pruning empty request/readiness and closing
pre-commit verification semantics; mandatory current verification at collection,
no-collection commitment, and
final truth closure; universal historical Binding resolution/validity plus the
explicit `historical_validity_only` / `continuing_current` reliance-time rule;
separate fresh action authorization; readiness states and refusal boundary;
diagnostic versus decision-seeking behavior; capability/ceiling/host/actual-
reach separation; retry, partial collection, cache, escalation, supersession,
and reuse rules; advisory versus enforceable behavior without a general
Handbook IAM mandate; the responsibility map; and every scenario outcome above.

Deliberately deferred to later human-approved task-level planning are concrete
generalized observation-free evaluation (including initially empty obligations
and retained observation-free claims), schemas, types, APIs, operations, storage, registries, transports, status-code
encodings, algorithms, cache mechanisms, retry counts, timeouts, producer
selection, capability policies, host constraints, enforcement mechanisms,
crate/module placement, migrations, feature flags, slice or packet
decomposition, acceptance planning, and rollout. Concrete assurance mappings,
requirements, metrics, thresholds, gate policies, concrete explanation-
projection decisions, UI, parent-promotion policy content, posture-transition
policy content, publication, and external adopter or Substrate selection remain
owned elsewhere or deferred. The proposed
[operational-reliance and work-progression decision](operational-reliance-and-actions.md#reliance-actions)
owns detailed present-use and action-chain semantics and proposes Work
Progression as local/parent eligibility owner; physical placement, work hierarchy
and state model, concrete policies, and implementation remain deferred.
