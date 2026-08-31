# HCM posture and assurance-policy semantic decision

**Status: PROPOSED**

This record fixes the product vocabulary, responsibility boundaries, resolution
rules, and discriminating scenarios for the Handbook Contract Membrane seam
from project posture to assurance requirements. It is semantic authority for
human review only. It is not implementation authority, a slice or packet
specification, or rollout planning.

The companion
[metric, evaluation, and gate semantic decision](metric-gate-semantic-decision.md)
owns metric and gate meaning. This record owns the earlier selection seam: how
an exact resolved posture may select exact assurance requirements that metrics
and gates later consume without reinterpreting posture.

## Canonical model and vocabulary

The canonical forward path is:

```text
explicit Charter posture + exact scope
  -> Resolved Posture Snapshot P_n
  -> assurance-policy selection
  -> Resolved Assurance Requirements
  -> contracts / evidence / claim evaluations
  -> named metrics
  -> semantic gate decision
  -> separately owned enforcement disposition
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
| **Resolved Assurance Requirements** | The immutable, fingerprinted output of applying one exact Assurance Policy to one Resolved Posture Snapshot. Its identity transitively binds the snapshot; exact selector identity, version, semantic-content identity, and currentness; exact admitted policy-resolution basis and currentness; exact selected-policy identity, version, semantic-content identity, and currentness; every admitted exception input; every other declared selection input; and complete candidate accounting and canonical explanation. Its output binds exact named/versioned contract or claim-set references, evidence policies, Metric Definitions, Gate Policy, and admitted exception requirements. Metrics and gates consume this result; they do not derive it. |
| **Metric Definition** | The immutable, named, versioned, fully configured algorithm definition owned by the companion record. Assurance selection may choose an exact definition reference; it MUST NOT change the definition's scale, unit, population, algorithm, parameters, missing-input policy, or numeric meaning. |
| **Gate Policy** | The immutable semantic decision rule owned by the companion record. Assurance selection may choose an exact policy reference; gate evaluation remains a pure application of that resolved policy to compatible claim evaluations and metric results. |
| **Enforcement Policy** | A separately owned, named, versioned rule that consumes an exact semantic gate decision, its Resolved Posture Snapshot, and the required evaluation provenance to determine an operational disposition. It cannot alter semantic gate truth or authorize posture mutation by implication. |
| **Enforcement Disposition** | The non-semantic operational interpretation produced by an Enforcement Policy, such as whether the separately owned workflow proceeds, blocks, or holds. Different enforcement policies may produce different dispositions from the same gate truth without changing that truth. |
| **Posture Observation** | An immutable, provenance-bound, admitted fact that may inform later posture reasoning, including an exact gate result when a named observation policy permits it. An observation is neither a recommendation nor authority to mutate posture. |
| **Posture Recommendation** | A separately owned, explainable proposal derived from admitted observations and a named recommendation policy. It identifies a proposed transition but cannot apply it, and its production does not imply approval. |
| **Posture Transition** | The separately authorized, atomic Engine-owned mutation from one exact current posture authority state to the next. It performs fresh authority and currentness checks and creates the next posture state; neither an Assurance Policy, gate, observation, recommendation, nor enforcement disposition may perform it. |

## Current product facts and semantic gap

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

Therefore no current free-form Charter text, classification heuristic, gate
result, or null profile field may be treated as an implemented assurance-policy
mapping.

## Resolution and selection invariants

### Classification does not control posture

- Project Classification MAY inform authoring-time defaults and suggestions.
- A suggestion MUST remain visibly provisional until persisted as explicit
  posture authority through the separately authorized Charter process.
- Persisted explicit posture controls resolution. Classification MUST NOT
  silently override, raise, lower, or relax a persisted baseline or dimension.
- Reclassification alone MUST NOT mutate posture or make an existing explicit
  posture disappear.

### Posture is a scoped dimension vector

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

### Assurance selection precedes evaluation

- Assurance-policy selection MUST occur before evidence selection, claim
  evaluation, metric computation, or gate evaluation for the run.
- The Assurance Policy Selector MUST consume only the exact Resolved Posture
  Snapshot and other exact, declared, admitted, and currentness-bound selection
  inputs. Project Classification, mutable posture or Charter state, free-form
  Charter prose, gate results, enforcement dispositions, and unbound ambient
  state MUST NOT become implicit selector inputs.
- The Assurance Policy Selector MUST resolve exactly one named, versioned,
  supported Assurance Policy or refuse. Missing, ambiguous, stale, substituted,
  or unsupported mapping MUST NOT fall back to a default policy.
- The selector MUST bind and completely account for the exact admitted
  policy-resolution basis it consulted, including every candidate or mapping
  considered and that basis's currentness. Changed selector content, resolution
  basis, selected-policy content, currentness, or another declared input MUST
  change the resolved identity or refuse even when the output references would
  otherwise look equivalent.
- The selected Assurance Policy MUST resolve exact named/versioned contract or
  claim-set references, evidence policies, Metric Definitions, Gate Policy,
  admitted exception requirements, and a canonical explanation.
- Resolved Assurance Requirements MUST completely account for selected,
  excluded, inapplicable, ambiguous, and refused candidates. Their identity
  MUST transitively bind the exact snapshot, selector, policy-resolution basis,
  selected policy, admitted exception inputs, all other declared inputs, the
  currentness of each currentness-bearing input, and that complete accounting.
  An omission MUST NOT become an implicit requirement or relaxation.
- Metrics and gates MUST consume the Resolved Assurance Requirements. They
  MUST NOT reread mutable posture, reinterpret levels or free-form text, select
  alternate requirements, or change the meaning of a Metric Definition.
- When posture influences selection, the evaluation context, its fingerprint
  and currentness basis, and its canonical explanation MUST bind the Resolved
  Posture Snapshot fingerprint; exact Assurance Policy Selector identity,
  version, semantic-content identity, and currentness; exact admitted
  policy-resolution basis identity and currentness; exact selected Assurance
  Policy identity, version, semantic-content identity, and currentness; and
  Resolved Assurance Requirements identity. The requirements identity
  transitively binds the admitted exception inputs, all other declared inputs,
  and complete selection accounting. Replay without those exact bindings MUST
  refuse.

This is the cross-boundary invariant shared with the
[metric/gate record](metric-gate-semantic-decision.md): **posture may select
which exact assurance requirements govern a run, but it may never alter the
meaning of the selected contracts, evidence policies, metrics, or gate, and no
metric or gate may reinterpret posture.**

### Trust invariants do not relax

At every numeric Posture Level, including level 1 (`Exploratory` in the current
authoring rubric), the following remain non-relaxable:

- authority and provenance checks;
- canonical admission and exact identity;
- complete claim, evidence, metric, condition, and selection accounting;
- deterministic evaluation and replay; and
- fail-closed behavior for absent, ambiguous, stale, incompatible,
  unauthorized, or unsupported inputs.

Posture may select among admitted requirements. It MUST NOT weaken the trust
needed to establish the inputs, identities, policy, or truth of that selection.

### Exceptions are typed authority

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

## Bounded monotonicity and vector order

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

## Causal order and run binding

The only permitted causal cycle across evaluations is:

```text
P_n Resolved Posture Snapshot
  -> Resolved Assurance Requirements
  -> contracts / metrics / gate
  -> semantic gate decision
  -> optional admitted Posture Observation
  -> separately owned Posture Recommendation
  -> separately authorized Engine Posture Transition
  -> P_{n+1}
```

- A run remains bound to `P_n` and its Resolved Assurance Requirements. A
  concurrent posture transition MUST NOT silently rebind or reselect that run.
- A fresh evaluation under `P_{n+1}` MUST perform fresh scope resolution and
  assurance selection. Results under `P_n` and `P_{n+1}` are distinct contexts
  even when their other inputs happen to match.
- A gate MUST NOT mutate posture, reselect the posture governing its own run,
  or emit a transition as part of pass/fail evaluation.
- Gate `pass`, `fail`, `indeterminate`, or `not_applicable` MUST NOT
  automatically raise or lower posture. A permitted gate result may become an
  admitted Posture Observation only through its separately owned policy.
- A Posture Recommendation remains non-mutating. The Engine transition owner
  MUST independently establish current authority, exact expected posture,
  approvals, and transition validity.

## Enforcement separation

Enforcement is downstream of semantic gate truth. An Enforcement Policy MAY
consume an exact gate decision, the exact Resolved Posture Snapshot, the exact
Resolved Assurance Requirements, and other inputs it declares. It MUST NOT
change a gate decision, replace `indeterminate` with `pass`, treat a disposition
as semantic evidence, or acquire posture-transition authority. The same gate
truth may legitimately yield different dispositions under different exact
Enforcement Policies; the explanation must preserve both identities and the
unmodified gate result.

## Responsibility map

| Responsibility | Semantic owner | MUST NOT own or infer |
| --- | --- | --- |
| Charter authoring and governance | Project authority authors and approvers own Project Classification, persisted baseline and dimension values, descriptive guidance, and authorized Charter changes. | Executable scope matching, assurance selection by prose, metric meaning, gate truth, or automatic posture mutation. |
| Posture resolution | The posture-resolution owner resolves exact scope and produces the immutable Resolved Posture Snapshot from current admitted Charter authority. | Classification-based override, assurance-policy invention, evidence interpretation, gate truth, or transition approval. |
| Assurance-policy authority | Establishes and resolves named/versioned selectors and Assurance Policies and produces Resolved Assurance Requirements with complete explanation. | Posture mutation, canonical evidence admission, metric reinterpretation, gate evaluation, or enforcement. |
| Exception governance, authority, and admission | Charter governance owns non-executable nominations and process declarations. The designated exception-policy authority owns admitted exception semantics, permitted issuers, scope, effects, record requirements, and currentness; the exact record authority authorizes each record; and the exception-admission owner establishes authority, identity, scope, currentness, and policy conformance before selection may apply the bounded effect. | Treating Charter prose, approver names, process metadata, record location, or minimum fields as executable authority; allowing assurance selection to mint, approve, broaden, infer, or self-authorize an exception. |
| Contract, evidence, and claim evaluation | The respective semantic owners admit exact contract/evidence authority and establish applicability and claim truth under the resolved requirements. | Rereading posture, changing the selected requirements, gate disposition, recommendation, or posture mutation. |
| Metric computation and gate evaluation | The owners defined by the companion record purely apply exact Metric Definitions and Gate Policy inside the closed evaluation context. | Assurance reselection, mutable-posture interpretation, enforcement, recommendation, or mutation. |
| Enforcement | A separately named Enforcement Policy owner maps preserved gate truth and exact context to an Enforcement Disposition. | Rewriting semantic gate truth or directly changing posture. |
| Posture observation | A separately owned admission boundary establishes immutable, provenance-bound observations from permitted sources. | Recommendation, approval, or mutation merely because an observation exists. |
| Posture recommendation | A separately owned recommendation policy derives an explainable proposal from admitted observations. | Applying, approving, or implying a transition. |
| Posture transition | The Engine posture-transition owner performs the separately authorized atomic mutation and creates the next authority state. | Recomputing prior gate truth or treating a gate result, observation, or recommendation as self-authorizing. |

## Discriminating scenario corpus

Each scenario assumes exact admitted identities and fixed inputs except where
the row names a failure.

| Scenario and inputs | Expected semantic outcome | Invariant proved |
| --- | --- | --- |
| **Mixed dimension levels without collapse.** A snapshot has a lower `speed_vs_quality` level, a higher `security_privacy` level, and distinct values for the other dimensions. A policy declares only those two dimensions relevant. | The snapshot retains all nine effective dimensions. Selection applies the two declared inputs independently; it does not use the vector's max, min, average, baseline label, or an invented project score. | Posture is a vector and policy declares its consumers. |
| **Classification versus explicit posture.** A project is classified `greenfield`, while its persisted `security_privacy` posture is explicit and higher than an authoring suggestion for that classification. | The explicit posture controls. Classification may explain a suggestion but cannot lower, replace, or mutate the persisted value. | Classification and posture are distinct authorities. |
| **PII adapter versus docs-only tool.** One exact scope binds a PII-handling adapter; another binds a docs-only tool. A third request says only “the project area” and could match either. | Each exact scope resolves independently under a supporting policy. The ambiguous request refuses before assurance selection; free-form domain text does not choose for it. | Scope is exact and fails closed. |
| **Lighter assurance selection and trust.** A supported Resolved Posture Snapshot selects a lighter admitted requirement set, but one candidate lacks authority provenance and another leaves expected claims unaccounted. | Selection or evaluation refuses. A lighter requirement set cannot waive authority, provenance, canonical admission, complete accounting, determinism, or fail-closed behavior. | Trust invariants are non-relaxable at every numeric level. |
| **Posture changes mid-run.** A run closes on snapshot `P_n`; an authorized transition creates `P_{n+1}` before the gate finishes. | The existing run remains bound to `P_n` and its resolved requirements. It does not reselect. A fresh run under `P_{n+1}` resolves a new snapshot and requirements; the two contexts cannot be combined. | Runs bind one immutable posture snapshot. |
| **Failed gate feeds posture reasoning.** A gate returns `fail`, and a named observation policy admits that exact result. A recommendation policy proposes raising a relevant dimension. | The gate truth remains `fail`; the admitted observation and recommendation are non-mutating. Only a separately authorized Engine transition can create the next posture. | Feedback does not create a causal mutation shortcut. |
| **Relevant-dimension monotonicity.** Within the same policy family/version and exact scope, all non-posture inputs and other dimensions are fixed while one declared relevant dimension increases. | The newly resolved requirements preserve or strengthen every prior requirement under the policy's declared strength relation; none is removed or weakened. | Monotonicity is bounded and dimension-relevant. |
| **Incomparable posture vectors.** For the same policy and scope, vector A is higher in security while vector B is higher in operability, and both dimensions are relevant. | Neither vector is declared globally stricter. The exact policy resolves each independently without scalar collapse; any unsupported combination refuses. | A dimension vector has no global total order. |
| **Missing policy mapping.** A valid snapshot and exact scope have no supported Assurance Policy mapping, or two selectors tie without deterministic precedence. | Selection refuses and produces no Resolved Assurance Requirements. It does not choose a default, nearest, lowest, or highest policy. | Missing or ambiguous mapping fails closed. |
| **Different enforcement, same truth.** The same exact semantic gate decision and context are consumed by two different named/versioned Enforcement Policies. | The exact semantic gate decision and canonical explanation remain unchanged. The policies may produce different dispositions, each attributed to its own identity, without rewriting the gate. | Enforcement is separate from gate semantics. |
| **Free-form shortcut and red line.** Charter text says a shortcut is allowed for a named area and a red line applies to another, but no admitted typed exception policy and record binds the exact scope. A second input is typed but was issued by an unauthorized actor; a third was synthesized or approved by the selector itself. | The prose and both invalid records refuse and do not change requirement selection. Only an exact record that was separately authorized and admitted under the exact exception policy may have its bounded effect applied. | Governance prose, typing alone, and selector self-authorization are not exception authority. |
| **Deterministic requirement replay.** The exact Charter identity/revision, scope, snapshot fingerprint, selector identity/version/content/currentness, admitted policy-resolution basis/currentness, selected-policy identity/version/content/currentness, admitted exception inputs, and other declared inputs are replayed. A comparison case uses a different policy or resolution basis that yields apparently equivalent requirement references. | Exact replay produces the identical Resolved Assurance Requirements identity, references, accounting, and explanation. The comparison case has a distinct provenance-bound requirements identity or refuses; equivalent-looking references do not collapse different authority. Any changed or missing binding likewise changes identity or refuses. | Resolved requirements are exact, transitively closed, and reproducible. |

## Settled versus deferred

This record settles the canonical terms, semantic ownership, classification and
posture separation, vector and scope model, snapshot closure, fail-closed
selection, evaluation provenance, non-relaxable trust invariants, bounded
monotonicity, causal ordering, enforcement separation, typed-exception boundary,
and scenario outcomes above.

Deliberately deferred to later human-approved task-level authority are concrete
policy mappings and requirement sets; thresholds; schema shapes; Rust types or
APIs; storage and registry mechanics; expression languages; migrations;
feature flags; slice or packet decomposition; rollout; and implementation
sequence. This record also does not select an assurance-policy owner placement,
observation or recommendation algorithm, acknowledgement or hysteresis model,
UI, publication, or external adopter.

Human approval of this proposed record would make it semantic input to later
planning only. It would not authorize implementation, change any phase or slice
status, or establish a delivery outcome.
