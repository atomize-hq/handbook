# HCM posture/assurance semantic decision — ChatGPT Pro review

**Date:** 2026-08-30

**Task/spec:** Independent semantic review and remediation of the proposed posture-to-assurance-policy decision record.

**Current approach:** Scoped posture vector -> immutable resolved snapshot -> exact assurance-policy selection -> closed resolved requirements -> pure contract/metric/gate evaluation -> separate enforcement and later posture feedback.

**Conversation:** https://chatgpt.com/c/6a94e417-accc-83ea-bd44-b3a35844109f

**Model:** GPT-5.6 Sol, ChatGPT Pro power 5/5.

**Artifact lineage:** Initial candidate commit 144db17; initial candidate blob d7686f0cf317d02cd9f6ca385ea5eb9c02de76a1; remediated candidate blob a56bffdd74765b8787785bdc31cac2606f542913. Initial companion metric blob 8ce7013c640713ba6ce042d293b95b446129744b; remediated companion metric blob 61ac86e90ef11693b911f7ac70f4dd68a225ecf7.

Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

Advisory context only; local repository inspection and validation determine final actions.

## Round 1 — independent approach review

### Prompt

~~~~text
We are already working on this project task.

Task/spec: Independently review a proposed semantic decision record for the Handbook Contract Membrane posture-to-assurance-policy seam. Decide whether it narrowly but completely fixes vocabulary, ownership, scope and posture resolution, assurance-policy selection provenance/currentness, non-relaxable invariants, bounded monotonicity, gate-to-posture feedback, enforcement separation, and the discriminating scenarios needed before later task-level planning. This is a semantic record only, not implementation authority.

Current approach: Repository-derived inference: retain today's typed Charter posture as a 1-5 baseline plus nine fixed dimensions; distinguish Project Classification from posture; model posture as an exact scoped vector; resolve it into one immutable snapshot; use an independently owned named/versioned Assurance Policy and selector to produce immutable Resolved Assurance Requirements before contract/evidence/metric/gate evaluation; keep metrics/gates pure; keep enforcement separate; and permit gate feedback to influence only a later posture state through admitted observation, recommendation, and separately authorized Engine mutation. The complete candidate is included below.

Project sources: No managed Project source set or repository bundle is configured. Treat only the bounded repository-truth packet and complete candidate below as available. Excluded repository source is unavailable and must not be inferred.

Evidence, blockers, constraints, and validation:
- Current repository truth: the canonical Charter schema fixes rubric_scale to 1-5, baseline_level to 1..5, exactly nine posture dimensions, and optional per-dimension 1..5 overrides. Project Classification is a separate five-value enum.
- The current private ProjectPostureKernel resolves the baseline/overrides and fingerprints the Charter/vector, but does not structurally resolve free-form domain_overrides or default_stance into executable scope/policy.
- Broader PostureEvaluationPolicy, multi-source recommendation, acknowledgement, and hysteresis behavior are absent; the shipped profile permits only null. A separately authorized atomic Engine-owned posture transition exists.
- Current Contracts gate/local-closeout truth is pure and contains no posture field or posture policy.
- Greenfield constraint: do not introduce V1/V2 or legacy framing.
- The candidate must not specify concrete mappings, thresholds, schema/Rust/API shapes, migrations, slice IDs, rollout, or implementation order.
- The candidate must not claim the free-form Charter domain/shortcut/red-line prose is executable.
- Non-relaxable trust includes authority/provenance, canonical admission, identity, complete accounting, deterministic evaluation/replay, and fail-closed behavior.
- Bounded monotonicity may be claimed only within one policy family/version, exact scope, and fixed other inputs. Posture vectors can be incomparable; there is no universal strictness scalar.
- Required causal order is P_n snapshot -> requirements -> contracts/metrics/gate -> optional admitted observation -> separately owned recommendation -> separately authorized Engine transition -> P_{n+1}; an in-flight run cannot rebind itself.
- Local validation passed: links, whitespace checks, PROPOSED status, no V1/V2 framing, exactly the intended three documentation files, and all 12 required scenarios.
- Companion metric/gate insertion at the document preamble: The companion posture and assurance-policy semantic decision owns the proposed upstream selection seam. When posture influences assurance selection, that record's exact snapshot and resolved-requirement provenance are part of the metric/gate record's closed evaluation context; neither record is implementation authority.
- Companion metric/gate insertion at evaluation context: When posture influenced assurance selection, the evaluation context fixes the Resolved Posture Snapshot fingerprint, Assurance Policy Selector identity/version, and Resolved Assurance Requirements identity. Those bindings participate in evaluation fingerprinting, currentness, and canonical explanation. Metrics and gates consume resolved requirements and never reread/reinterpret mutable posture, reselect requirements, or change metric meaning.
- README adds one navigation row identifying this proposed record and explicitly denying implementation, slice, or rollout authority.

Review requirements:
1. Return findings first by P1, P2, P3, or P4. For each, identify the exact section/text, violated semantic invariant, material downstream consequence, and the smallest correction.
2. Treat P1/P2 as blocking. Use P3 for a real semantic ambiguity likely to cause incompatible implementation plans. Use P4 only for useful non-blocking clarity. Do not manufacture a finding to fill a severity.
3. Stress-test especially: Project Classification versus posture; baseline versus effective dimensions; scope identity/currentness; selector versus selected policy ownership; requirement-set closure; exception authority; metric/gate purity; enforcement's ability to consume posture without rewriting gate truth; bounded monotonicity over partial vector order; and the P_n to P_{n+1} feedback boundary.
4. Detect accidental implementation claims or missing provenance that would let two conforming implementations disagree materially.
5. If a suggested change depends on repository facts not in this packet, label it as a question rather than asserting it.
6. Then give a direct KEEP, ADJUST, or PIVOT verdict. State explicitly if there are no qualifying findings. For ADJUST, list exact minimal edits; for KEEP, identify why the seam is sufficiently complete; for PIVOT, identify the flawed model.
7. Do not propose concrete thresholds, mappings, APIs, schemas, or task decomposition; those are deliberately deferred.

Complete candidate artifact follows:

--- BEGIN posture-assurance-policy-semantic-decision.md ---
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
| **Posture Level** | One value on the Charter's fixed `1-5` rubric, used as a persisted baseline or an explicit value for one Posture Dimension. A level has meaning only for its named dimension or for baseline inheritance; it is not a universal project strictness score. |
| **Posture Dimension** | One named, independent axis of engineering posture. The Charter currently has exactly nine ordered dimensions. A project's posture is the vector of their effective levels, not their maximum, minimum, average, or another ambient scalar. |
| **Posture Scope** | The exact bounded subject and applicability boundary for which posture is being resolved, including every identity and currentness binding required to distinguish that subject from another domain, component, operation, or work context. The posture-resolution owner owns this binding and refuses absent, ambiguous, stale, or unsupported scope. |
| **Resolved Posture Snapshot** | The posture-resolution owner's immutable, fingerprinted determination for one exact Posture Scope. It binds the exact Charter identity and revision, persisted baseline, effective value of every Posture Dimension, exact scope, resolution inputs, and currentness basis. It is the only posture form an Assurance Policy may consume. |
| **Assurance Policy** | An immutable, named, versioned semantic rule that declares the exact Posture Scope it supports, the Posture Dimensions it consumes, and how a supported Resolved Posture Snapshot selects assurance requirements. The assurance-policy authority owns the rule; it does not own posture mutation, evidence admission, metric computation, gate truth, or enforcement. |
| **Assurance Policy Selector** | The immutable, named, versioned selection rule that resolves exactly one applicable Assurance Policy or refuses. Its identity and version are distinct provenance even when packaged with the selected policy. |
| **Resolved Assurance Requirements** | The immutable, fingerprinted output of applying one exact Assurance Policy to one Resolved Posture Snapshot. It binds exact named/versioned contract or claim-set references, evidence policies, Metric Definitions, Gate Policy, admitted exception requirements, complete selection accounting, and canonical explanation. Metrics and gates consume this result; they do not derive it. |
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
  with the five values named above.
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
- The Assurance Policy Selector MUST resolve exactly one named, versioned,
  supported Assurance Policy or refuse. Missing, ambiguous, stale, substituted,
  or unsupported mapping MUST NOT fall back to a default policy.
- The selected Assurance Policy MUST resolve exact named/versioned contract or
  claim-set references, evidence policies, Metric Definitions, Gate Policy,
  admitted exception requirements, and a canonical explanation.
- Resolved Assurance Requirements MUST completely account for selected,
  excluded, inapplicable, and refused candidates. An omission MUST NOT become
  an implicit requirement or relaxation.
- Metrics and gates MUST consume the Resolved Assurance Requirements. They
  MUST NOT reread mutable posture, reinterpret levels or free-form text, select
  alternate requirements, or change the meaning of a Metric Definition.
- When posture influences selection, the evaluation context, its fingerprint
  and currentness basis, and its canonical explanation MUST bind the Resolved
  Posture Snapshot fingerprint, Assurance Policy Selector identity/version,
  and Resolved Assurance Requirements identity. Replay without those exact
  bindings MUST refuse.

This is the cross-boundary invariant shared with the
[metric/gate record](metric-gate-semantic-decision.md): **posture may select
which exact assurance requirements govern a run, but it may never alter the
meaning of the selected contracts, evidence policies, metrics, or gate, and no
metric or gate may reinterpret posture.**

### Trust invariants do not relax

At every Posture Level, including Exploratory, the following remain
non-relaxable:

- authority and provenance checks;
- canonical admission and exact identity;
- complete claim, evidence, metric, condition, and selection accounting;
- deterministic evaluation and replay; and
- fail-closed behavior for absent, ambiguous, stale, incompatible,
  unauthorized, or unsupported inputs.

Posture may select among admitted requirements. It MUST NOT weaken the trust
needed to establish the inputs, identities, policy, or truth of that selection.

### Exceptions are typed authority

- Free-form shortcut, red-line, stance, domain-override, rationale, or other
  Charter prose MUST NOT execute as an exception.
- Only an admitted, typed, named, versioned exception policy and its exact
  authorized exception record may affect requirement selection.
- The selection explanation MUST identify every applied exception and its
  authority, scope, identity, currentness, and effect. A missing or invalid
  exception MUST fail closed rather than become an implicit waiver.

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
| **Exploratory posture and trust.** An Exploratory scope would select a lighter admitted requirement set, but one candidate lacks authority provenance and another leaves expected claims unaccounted. | Selection or evaluation refuses. Exploratory posture cannot waive authority, provenance, canonical admission, complete accounting, determinism, or fail-closed behavior. | Trust invariants are non-relaxable. |
| **Posture changes mid-run.** A run closes on snapshot `P_n`; an authorized transition creates `P_{n+1}` before the gate finishes. | The existing run remains bound to `P_n` and its resolved requirements. It does not reselect. A fresh run under `P_{n+1}` resolves a new snapshot and requirements; the two contexts cannot be combined. | Runs bind one immutable posture snapshot. |
| **Failed gate feeds posture reasoning.** A gate returns `fail`, and a named observation policy admits that exact result. A recommendation policy proposes raising a relevant dimension. | The gate truth remains `fail`; the admitted observation and recommendation are non-mutating. Only a separately authorized Engine transition can create the next posture. | Feedback does not create a causal mutation shortcut. |
| **Relevant-dimension monotonicity.** Within the same policy family/version and exact scope, all non-posture inputs and other dimensions are fixed while one declared relevant dimension increases. | The newly resolved requirements preserve or strengthen every prior requirement under the policy's declared strength relation; none is removed or weakened. | Monotonicity is bounded and dimension-relevant. |
| **Incomparable posture vectors.** For the same policy and scope, vector A is higher in security while vector B is higher in operability, and both dimensions are relevant. | Neither vector is declared globally stricter. The exact policy resolves each independently without scalar collapse; any unsupported combination refuses. | A dimension vector has no global total order. |
| **Missing policy mapping.** A valid snapshot and exact scope have no supported Assurance Policy mapping, or two selectors tie without deterministic precedence. | Selection refuses and produces no Resolved Assurance Requirements. It does not choose a default, nearest, lowest, or highest policy. | Missing or ambiguous mapping fails closed. |
| **Different enforcement, same truth.** The same exact semantic gate decision and context are consumed by two different named/versioned Enforcement Policies. | The gate decision and explanation remain byte-for-byte semantic truth. The policies may produce different dispositions, each attributed to its own identity, without rewriting the gate. | Enforcement is separate from gate semantics. |
| **Free-form shortcut and red line.** Charter text says a shortcut is allowed for a named area and a red line applies to another, but no admitted typed exception policy and record binds the exact scope. | Neither text executes or changes requirement selection. Selection follows the admitted policy or refuses if an exception is required. | Prose is not executable policy or exception authority. |
| **Deterministic requirement replay.** The exact Charter identity/revision, scope, snapshot fingerprint, selector identity/version, policy, admitted exception inputs, and other selection inputs are replayed. | Replay produces the identical Resolved Assurance Requirements identity, references, accounting, and explanation. A changed or missing binding refuses rather than reproducing an apparently equivalent result. | Resolved requirements are exact, closed, and reproducible. |

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

--- END posture-assurance-policy-semantic-decision.md ---

Do not reduce the task or replace it with an easier alternative. Preserve the requested scope and project conventions.

Is the current approach the best way forward? Give a direct keep, adjust, or pivot recommendation; tradeoffs; what to keep/change first; inspections; validation; and pitfalls.
~~~~

### Response

~~~~text
Worked for 14m 36s
Findings
P1 — None

No P1 finding. The proposed model is coherent and does not require a pivot.

P2 — The assurance-selection result is not fully closed over the act of selection

Exact sections: the definitions of Assurance Policy, Assurance Policy Selector, and Resolved Assurance Requirements; “Assurance selection precedes evaluation”; the companion metric/gate insertion; and “Deterministic requirement replay.”

Pasted markdown +3

Violated semantic invariant: exact identity, complete selection accounting, deterministic replay, and assurance-policy currentness.

The record makes the selector and selected policy distinct semantic artifacts, but the normative binding list for Resolved Assurance Requirements does not explicitly close over:

the selector’s exact content identity or fingerprint, rather than only name/version;

the exact admitted candidate-policy or mapping set consulted by the selector;

that candidate set’s currentness basis;

the exact selected Assurance Policy identity, version, content fingerprint, and currentness;

the complete declared input set used by the selector.

The evaluation context then explicitly binds the posture snapshot, selector identity/version, and resolved-requirements identity, but not the selected policy or selection-basis identity. That is safe only if the resolved-requirements identity is explicitly defined as transitively binding all of them; the record currently leaves that transitive closure implicit.

The selector also is not expressly limited to a closed, declared set of admitted inputs. The forward path strongly implies that it consumes the resolved snapshot, and the classification section limits classification to authoring suggestions, but there is no direct selector invariant forbidding an implementation from consulting classification, mutable Charter state, free-form prose, or ambient registry state during selection.

Material downstream consequence: two conforming implementations could materially disagree while producing apparently identical resolved requirements:

one could select against a different candidate-policy catalog;

one could accept changed policy bytes under the same name/version;

two different selected policies could produce the same requirement references and collapse to one unresolved provenance identity;

a selector could silently use Project Classification or ambient mutable state;

replay could reproduce equivalent requirements without reproducing the exact selection authority or currentness basis.

That would make policy substitution, stale mapping, and same-output/different-authority cases impossible to distinguish reliably.

Smallest correction:

State that an Assurance Policy Selector may consume only the exact Resolved Posture Snapshot and exact admitted selection inputs it declares.

State that Project Classification, mutable posture, free-form Charter prose, gate results, enforcement dispositions, and unbound ambient registry state are not implicit selector inputs.

Extend the Resolved Assurance Requirements binding list to include:

Resolved Posture Snapshot fingerprint;

exact selector name/version/content identity;

exact admitted candidate-policy or mapping-set identity and currentness;

exact selected Assurance Policy name/version/content identity and currentness;

all other declared selection inputs;

complete candidate accounting and explanation.

Add the selected policy and exact selection basis to the companion evaluation-context sentence, or explicitly state there that the Resolved Assurance Requirements identity transitively binds them.

Expand the deterministic-replay scenario so that two policies producing identical requirement references nevertheless produce distinct provenance-bound resolved-requirements identities.

This correction does not require specifying a registry schema, API, storage mechanism, or concrete mapping.

P2 — Exception authority is required, but its semantic ownership is not assigned

Exact sections: “Exceptions are typed authority” and the responsibility map.

Pasted markdown +1

Violated semantic invariant: authority/provenance must remain non-relaxable, especially where an exception can remove or alter an otherwise selected requirement.

The record correctly requires an admitted, typed, named, versioned exception policy and an exact authorized exception record. It also requires the explanation to preserve authority, scope, identity, currentness, and effect. But the responsibility map does not assign the semantic responsibilities for:

defining exception semantics and permitted effects;

determining who may authorize an exception;

issuing or approving an exact exception record;

admitting and validating that record;

applying its bounded effect during assurance selection.

As written, a later plan could let the assurance-policy authority or selector mint, authorize, validate, and apply its own waiver. Another plan could assign authorization to Charter authors. Both could claim compliance because the record requires an “authorized” record without fixing what authority makes it authorized.

Material downstream consequence: implementations can disagree about who possesses waiver authority and whether the selection owner may self-authorize relaxation. That is not merely component placement; it changes the trust model and the meaning of an admitted exception.

Smallest correction:

Add one responsibility-map row and one invariant distinguishing these semantic acts:

Exception authority and admission — A designated exception-policy authority owns admissible exception semantics, scope, authorized issuer classes, and permitted effects. An exact exception-record authority authorizes each record. Assurance selection may validate and apply an admitted record but MUST NOT mint, approve, broaden, infer, or self-authorize it.

Concrete crate, service, or repository placement may remain deferred. One eventual component may host multiple responsibilities, but the authorities, acts, identities, and provenance must remain distinct.

Also expand the existing shortcut/red-line scenario to say that a typed record issued by an unauthorized actor—or synthesized by the selector—refuses just as free-form prose does.

P3 — Exploratory introduces an undefined scalar posture label

Exact sections: “Trust invariants do not relax” and the “Exploratory posture and trust” scenario.

Pasted markdown +1

Violated semantic invariant: the vocabulary is supposed to be closed around numeric 1-5 Posture Levels and a nine-dimensional effective vector, without introducing a project-wide posture class or implicit mapping.

Exploratory is capitalized and used as though it were a defined Posture Level or scope category, but it is not included in the canonical vocabulary and no numeric or vector mapping is supplied. Under the bounded packet, there is no source authority establishing that label.

Material downstream consequence: a planner could introduce an Exploratory enum, scalar posture class, or implicit label-to-level mapping, contradicting both the vector model and the deliberate deferral of concrete mappings.

Smallest correction:

Replace:

At every Posture Level, including Exploratory

with:

At every supported numeric Posture Level, including levels that select a lighter admitted requirement set

And rename/rewrite the scenario as:

Lighter assurance selection and trust. A supported Resolved Posture Snapshot selects a lighter admitted requirement set…

No new label or mapping is needed.

P4 — “Byte-for-byte semantic truth” accidentally asserts a representation-level guarantee

Exact section: “Different enforcement, same truth.”

Pasted markdown

Violated semantic invariant: this is semantic authority, not serialization or schema authority.

“Byte-for-byte” requires a defined canonical representation and comparison boundary. This record explicitly defers schema and implementation shapes, so that phrase is stronger and lower-level than the record can presently authorize.

Material downstream consequence: a later plan could interpret the scenario as requiring stable serialization bytes rather than preservation of the exact semantic gate-decision identity and explanation.

Smallest correction:

Replace:

remain byte-for-byte semantic truth

with:

remain the exact same semantic gate decision and canonical explanation

No other P3 or P4 findings qualify.

Verdict: ADJUST

The current model is the best way forward. The adjustment is narrow and does not change the architecture:

keep posture as an exact scoped vector rather than a scalar;

keep Project Classification outside runtime posture authority;

keep one immutable Resolved Posture Snapshot per run;

keep assurance selection upstream of contracts, evidence, metrics, and gates;

keep Metric Definitions and Gate Policies immutable and semantically pure;

keep trust invariants non-relaxable;

keep monotonicity bounded by policy family/version, scope, and fixed non-posture inputs;

keep gate feedback outside the in-flight evaluation and behind observation, recommendation, and separately authorized transition;

keep enforcement downstream and unable to rewrite gate truth.

Those boundaries are stated consistently and are exercised by useful discriminating scenarios.

Pasted markdown +1

The two P2 corrections must be made before later task-level planning. Otherwise, plans can disagree about the exact selection authority and about who can authorize requirement relaxation.

Tradeoffs

The selected architecture deliberately accepts more provenance and policy-authoring complexity than a scalar “posture score” or ambient default system. That is the correct trade:

Exact selection closure creates more identities and currentness bindings, but prevents same-name substitution, stale mappings, and unexplained policy changes.

Typed exception authority adds governance friction, but prevents the selection mechanism from becoming a waiver mechanism.

Vector posture makes comparison and testing more involved, but preserves materially different security, operability, testing, performance, and delivery positions that a scalar would destroy.

Pure gate truth plus separate enforcement allows different operational workflows without corrupting semantic evaluation.

P_n run binding means posture improvements do not retroactively repair an in-flight run, but it preserves replay and causal explainability.

These costs are appropriate for a contract-membrane authority boundary.

Exact minimal edit order

Close the selection identity first. Amend the selector, Resolved Assurance Requirements, evaluation-context, and deterministic-replay text so the exact candidate basis and selected policy are unambiguously bound.

Assign exception responsibilities second. Add the responsibility-map row and prohibit the selector or resolver from minting, approving, or broadening an exception.

Remove the undefined label. Replace both uses of Exploratory with language about a supported numeric/vector posture selecting lighter admitted requirements.

Remove the representation claim. Replace “byte-for-byte” with preservation of the exact semantic decision identity and explanation.

Do not alter the vector model, causal cycle, enforcement separation, or deferred implementation boundary. The record correctly leaves concrete mappings, thresholds, schemas, APIs, storage, rollout, owner placement, and task decomposition to later authority.

Pasted markdown

Required inspections before approval
1. Selection-identity closure inspection

For one hypothetical evaluation, trace every identity and currentness binding through:

Charter authority → exact scope → posture resolution → snapshot → selector → candidate-policy basis → selected Assurance Policy → exception inputs → Resolved Assurance Requirements → contract/evidence/metric/gate context.

Every arrow must have an exact admitted input and no ambient lookup whose identity is omitted from the resulting provenance.

2. Same-output, different-authority inspection

Exercise these cases semantically:

two different Assurance Policies resolve identical requirement references;

the same policy name/version is presented with different canonical content;

the selector’s candidate mapping set changes while the snapshot remains unchanged;

the selector name/version remains unchanged but its canonical rule content changes.

All must either produce distinct exact contexts or refuse. They must not collapse merely because the resulting requirement list looks equivalent.

3. Selector-input inspection

Verify explicitly that:

reclassification alone cannot affect assurance selection when explicit posture is unchanged;

persisted baseline is only an inheritance source and provenance value, not a tenth runtime posture dimension;

effective dimension values, not a baseline label, drive posture-based selection;

free-form domain_overrides, default_stance, shortcuts, and red lines cannot enter selector evaluation;

any permitted non-posture selection input is exact, declared, admitted, and currentness-bound.

4. Exception-authority inspection

Test four distinct semantic cases:

valid policy plus valid record from an authorized issuer;

valid typed record from an unauthorized issuer;

selector-generated or selector-approved record;

free-form Charter prose resembling an exception.

Only the first may affect requirement selection, and its exact bounded effect must be explained.

5. Vector and monotonicity inspection

Retain the candidate’s policy-family/version, exact-scope, and fixed-other-input boundary. Validate:

one relevant dimension increases while all other consumed dimensions remain fixed;

several consumed dimensions increase componentwise;

relevant dimensions move in opposing directions and remain incomparable;

undeclared dimensions change without being implicitly interpreted;

policy version, scope, or non-posture input changes are not mislabeled as monotonic comparisons.

The current bounded-monotonicity model is otherwise sound and correctly rejects a universal strictness scalar.

Pasted markdown

6. Feedback and enforcement inspection

Confirm that:

a run under P_n never rebinds after P_{n+1} exists;

a gate result becomes a Posture Observation only through its exact admission policy;

a recommendation remains tied to its source observations and proposed transition;

the Engine performs fresh expected-state and authority checks;

two Enforcement Policies may produce different operational dispositions while preserving the exact same gate decision.

The proposed causal cycle and enforcement boundary are correctly drawn.

Pasted markdown

Validation after editing

The reported local validation establishes document scope, links, whitespace, status, vocabulary exclusions, and the presence of twelve scenarios, but it cannot establish selection closure or authority completeness.

Pasted markdown

After the edits:

Re-run the existing three-file path, link, whitespace, status, and forbidden-framing checks.

Preserve the twelve-scenario count by expanding existing rows rather than necessarily adding new rows:

expand deterministic replay with same-output/different-policy provenance;

expand the shortcut/red-line scenario with unauthorized and selector-minted records;

rewrite the Exploratory scenario without adding a posture label;

rewrite the enforcement row without “byte-for-byte.”

Perform a terminology pass confirming every capitalized domain term is either defined or clearly non-normative.

Confirm that the candidate and companion metric/gate text name the same closed evaluation bindings.

Confirm that no correction introduces a registry schema, API, threshold, policy mapping, owner crate, migration, implementation sequence, or rollout claim.

Principal pitfalls

Treating name/version as sufficient identity while allowing changed canonical content.

Accounting completely over an unspecified or mutable candidate universe.

Allowing Project Classification or Charter prose to become an undeclared selector input.

Treating equivalent requirement lists as equivalent policy provenance.

Letting the selection owner create or approve its own exception.

Reintroducing a scalar posture through labels such as Exploratory.

Applying monotonicity across policy versions, scopes, or changed exception inputs.

Letting a later posture state rebind an in-flight evaluation.

Treating Enforcement Disposition as gate truth or as posture-mutation authority.

Turning this semantic adjustment into premature schema, registry, or task-decomposition design.

Sources
~~~~

## Round 2 — bounded repository-truth clarification

### Prompt

~~~~text
Bounded repository-truth clarification before remediation. Please reconsider only the affected findings and return a revised final finding set; do not broaden the review.

The prior packet omitted two source facts:

1. The current authoring source defines the five rubric labels explicitly:
- level 1: Exploratory — throwaway ok; optimize learning; minimal gates
- level 2: Prototype — demoable/internal use; some structure; still speed-first
- level 3: Product — real users; balanced; maintainability matters
- level 4: Production — GA/customer-facing; strong quality/reliability/security defaults
- level 5: Hardened — critical/regulated/high blast radius; strict gates; defense-in-depth
The persisted schema remains numeric 1..5; these are existing repository labels, not a proposed new posture class.

2. The canonical Charter already carries governance fields: decision_authority, required_approvals, a free-form exception_policy, and exception_process with approvers, record_location, and minimum_fields. Those fields are structurally validated/canonicalized as Charter authority, but their prose/list contents are not an executable typed exception policy, admitted exception record, or assurance selector. The current product has no executable exception-policy/record admission mechanism for this seam.

Please answer exactly:
A. Does the P3 Exploratory finding survive this source fact? If not, explicitly withdraw it. If a smaller P4 clarity edit remains useful, say whether it should define the source label as a label for numeric level 1 or avoid labels in the invariant/scenario, and why.
B. Does the exception-authority P2 remain? Refine the smallest semantic ownership language so it respects the current Charter governance facts without accidentally treating Charter prose or approver names as executable admission. State whether Charter governance may nominate authorization policy/issuer classes while exact typed policy, record authorization, and admission/application remain distinct acts.
C. Confirm or revise the exact minimal remediation set, including the selection-closure P2 and representation-level P4. Identify any prior finding now withdrawn.
D. Identify any remaining recommendation that depends on unavailable repository facts; phrase it as a question, not an assertion.

Do not reduce the original review scope, but do not introduce schemas, APIs, mappings, owner crates, task decomposition, or implementation planning.
~~~~

### Response

~~~~text
Worked for 6m 10s
A. Exploratory finding

The P3 finding does not survive the clarified repository fact. I withdraw it in full.

Because Exploratory is already the repository-defined authoring label for numeric level 1, its use does not introduce a new posture class, scalar, or implicit label-to-level mapping. The candidate continues to define runtime posture as the effective nine-dimensional vector and expressly rejects collapsing that vector to a label or universal score.

Pasted markdown +1

A smaller P4 wording clarification remains useful, but only for precision:

Define once that Exploratory is the current authoring-rubric label for numeric level 1, not a separate posture type.

Prefer numeric/vector wording in the normative invariant and scenario rather than using the bare label.

The smallest edits would be:

At every numeric Posture Level, including level 1 (Exploratory in the current authoring rubric), the following remain non-relaxable…

And replace:

An Exploratory scope would select a lighter admitted requirement set…

with:

A supported Resolved Posture Snapshot selects a lighter admitted requirement set…

The reason is not that Exploratory is undefined. It is that a Posture Scope is an applicability boundary, not a level, and “Exploratory scope” can momentarily read as though the scope itself carries a project-wide posture class. The numeric/vector wording preserves the repository label without weakening the candidate’s distinction between scope, baseline inheritance, and effective dimension values.

Pasted markdown +2

This is non-blocking clarity only.

B. Exception-authority P2

The exception-authority P2 remains.

The new facts establish that the Charter already owns canonical governance declarations concerning decision authority, required approvals, exception-process guidance, approvers, record location, and minimum fields. They do not establish executable exception-policy semantics, exact per-record authorization, record admission, or bounded application during assurance selection.

That distinction is already directionally consistent with the candidate’s rule that Charter prose cannot execute as an exception.

Pasted markdown +1

 What remains missing is an explicit separation of the semantic acts by which a governance nomination becomes an admitted exception that may affect requirements.

Refined smallest ownership language

The smallest correction should state:

Exception governance, authorization, admission, and application are distinct semantic acts. Canonical Charter governance may nominate an authorization policy, required approval or issuer classes, process expectations, record location, and minimum authoring fields. Those declarations are governance inputs only; Charter prose, approver names, and process metadata do not themselves constitute an executable typed exception policy, authorize a particular exception record, or admit an exception for assurance selection. An exact typed, named, versioned exception policy governs permitted issuers, scope, effects, and record requirements. Each exact exception record must be separately authorized under that policy and admitted through an exact authority, identity, scope, currentness, and policy-conformance check. Assurance selection may apply only the admitted bounded effect and may not mint, approve, broaden, infer, or self-authorize an exception.

The responsibility map should assign those responsibilities abstractly without requiring separate components:

One eventual component may host more than one responsibility, but Charter governance nomination, typed-policy authority, exact record authorization, record admission, and selection-time application MUST remain semantically distinct.

Therefore, yes: Charter governance may nominate an authorization policy and issuer or approver classes. That nomination remains non-executable until resolved through exact typed policy authority. Exact policy admission, authorization of a particular record, admission of that record, and application of its bounded effect remain distinct acts.

The material risk remains that, without this distinction, one plan could treat canonicalized Charter prose or a listed approver name as sufficient executable authorization, while another could require a typed policy and exact admitted record. Both might claim compliance with the current wording, but they would implement materially different trust boundaries. The current responsibility map does not yet close that ambiguity.

Pasted markdown

C. Revised final finding set
P1 — None

No P1 finding.

P2 — Assurance-selection provenance is not fully closed

Exact sections: the definitions of Assurance Policy, Assurance Policy Selector, and Resolved Assurance Requirements; “Assurance selection precedes evaluation”; the companion evaluation-context insertion; and “Deterministic requirement replay.”

Pasted markdown +3

Violated invariant: exact identity, complete selection accounting, currentness, and deterministic replay.

The candidate binds the snapshot, selector name/version, and resulting Resolved Assurance Requirements identity, but it does not yet state unambiguously that the result is closed over:

the selector’s exact semantic content identity, not merely its name/version;

the exact admitted policy-resolution basis consulted by the selector, including the applicable candidates or mappings considered;

that resolution basis’s currentness;

the exact selected Assurance Policy identity, content identity, and currentness;

every other declared selector input;

the exclusion/refusal accounting necessary to show why that policy, rather than another, was selected.

It also does not expressly prohibit the selector from consulting undeclared mutable or free-form inputs.

Material consequence: two implementations could consult different policy-resolution bases, accept changed selector or policy content under the same names/versions, or select different policies that happen to produce equivalent requirement references. Their outputs could appear identical while embodying different authority and provenance.

Smallest correction:

Require the selector to consume only the exact Resolved Posture Snapshot and other exact admitted inputs it declares.

Exclude Project Classification, mutable posture, free-form Charter prose, gate results, enforcement dispositions, and unbound ambient state as implicit selector inputs.

Require the Resolved Assurance Requirements identity to transitively bind:

the snapshot;

exact selector identity/content/currentness;

exact admitted policy-resolution basis and its currentness;

exact selected Assurance Policy identity/content/currentness;

admitted exception inputs;

all other declared inputs;

complete selection, exclusion, inapplicability, ambiguity, and refusal accounting.

Either list those bindings in the companion evaluation context or expressly state that the Resolved Assurance Requirements identity transitively closes over them.

Expand deterministic replay to distinguish two different policies or selection bases that produce apparently equivalent requirement references.

No schema, registry, API, or storage mechanism needs to be selected.

P2 — Exception governance does not yet close exact authorization, admission, and application ownership

Exact sections: “Exceptions are typed authority,” the free-form Charter exclusions, the responsibility map, and the shortcut/red-line scenario.

Pasted markdown +3

Violated invariant: authority and provenance cannot relax merely because governance prose or approver metadata exists.

The Charter facts supply legitimate governance nomination and process authority. They do not assign the semantic acts by which an exact typed policy becomes applicable, a particular record becomes authorized, that record becomes admitted, and its bounded effect becomes applicable during selection.

Material consequence: implementations could disagree over whether:

a free-form exception_policy field is itself executable;

an approver name is sufficient exact issuer authority;

satisfying minimum_fields makes a record admitted;

the selector may validate and authorize its own waiver;

record location implies authenticity or currentness.

Smallest correction: add the refined ownership language from section B, expressly preserving Charter governance as a nomination source while separating typed-policy authority, exact record authorization, admission, and application.

P3 — None

The prior P3 Exploratory finding is withdrawn because the repository authoring source already defines Exploratory as the label for numeric level 1.

P4 — Clarify the authoring label without treating scope as a level

Exact text: “At every Posture Level, including Exploratory” and “An Exploratory scope…”

Pasted markdown +1

Clarity issue: the label is valid, but the record should not use it as though it names a Posture Scope or a project-wide posture category.

Smallest correction: define Exploratory once as the current authoring label for numeric level 1, then use numeric/vector or “lighter admitted requirement set” wording in the invariant and scenario.

P4 — Remove the representation-level “byte-for-byte” guarantee

Exact text: “The gate decision and explanation remain byte-for-byte semantic truth.”

Pasted markdown

Clarity issue: “byte-for-byte” asserts a canonical serialization and byte-comparison boundary even though representation, schema, and API shapes are deliberately deferred.

Smallest correction:

The exact semantic gate decision and canonical explanation remain unchanged.

or:

The gate decision retains the same exact semantic identity and explanation.

Revised verdict: ADJUST

The verdict remains ADJUST. The architecture remains the correct model; the required changes are bounded closure and authority corrections, not a pivot.

Exact minimal remediation set

The blocking remediation remains:

Close assurance selection over its exact inputs and authority

exact selector identity/content/currentness;

exact admitted policy-resolution basis and currentness;

exact selected-policy identity/content/currentness;

declared-input closure;

transitive binding into Resolved Assurance Requirements and the evaluation context;

same-output/different-policy replay discrimination.

Separate exception governance, typed-policy authority, exact record authorization, admission, and application

preserve canonical Charter governance as a possible nomination source;

expressly deny executable force to Charter prose, approver names, record location, or minimum-field metadata by themselves;

prohibit selector-minted, selector-approved, inferred, or broadened exceptions.

The complete minimal non-blocking cleanup is:

Anchor Exploratory to numeric level 1 once and avoid “Exploratory scope.”

Replace “byte-for-byte semantic truth” with exact semantic identity/unchanged-truth wording.

The former P3 is the only prior finding withdrawn.

D. Remaining repository-dependent questions

None of the semantic corrections above depends on unavailable repository facts. They can remain abstract and owner-placement-neutral.

The following naming or reuse decisions do depend on repository facts not present in the bounded clarification, so they should remain questions:

Does an existing canonical repository authority already own exact typed exception-policy identity, per-record authorization, or exception-record admission semantics that this record should reference, or must the record assign those responsibilities abstractly without naming an existing placement?

Does an existing policy authority already define the exact content identity and currentness rules for named/versioned selectors, selected policies, and their policy-resolution basis, such that this record should reuse that established terminology rather than introduce parallel identity language?

Those questions affect how the semantic responsibilities are referenced, not whether the two P2 corrections are required.

Sources
Is this conversation helpful so far?
~~~~

## Round 3 — remediation follow-up

### Prompt

~~~~text
Review mode: remediation-follow-up
Review context: explicitly requested continuity in this same independent consultation
Review boundary: previously reviewed posture artifact blob d7686f0cf317d02cd9f6ca385ea5eb9c02de76a1 -> current blob a56bffdd74765b8787785bdc31cac2606f542913; companion metric artifact blob 8ce7013c640713ba6ce042d293b95b446129744b -> current blob 61ac86e90ef11693b911f7ac70f4dd68a225ecf7
Review target: only the remediation of the accepted findings in the complete current semantic artifact and exact companion evaluation-context paragraph below
Supported inputs and behavior: proposed normative Markdown semantics for posture resolution, assurance-policy selection, exception authority, metric/gate evaluation context, and twelve discriminating scenarios; no executable parser, schema, API, or implementation is in scope
Supported platforms and dialects: the local Handbook Contract Membrane semantic authority described in the bounded packet; no external platform, unpublished consumer, or alternate policy dialect may be inferred
In-scope invariants: exact and transitive selection provenance/currentness; selector input closure; complete candidate accounting; distinction among Charter governance nomination, typed exception-policy authority, exact record authorization, record admission, and bounded application; Project Classification/posture separation; vector posture; metric/gate purity; unchanged semantic gate truth under enforcement; P_n run binding; no implementation authority
Required material consequence: a blocking finding must show that two conforming downstream plans could still disagree materially about selection authority/currentness, exception authorization/admission/application, metric/gate truth, or another accepted semantic boundary
Blocking threshold: P1 or P2 only, with a reachable ambiguity in the current text and the material consequence above; P3/P4 may identify real non-blocking clarity or consistency regressions caused by the remediation
Accepted prior findings: (1) assurance-selection provenance was not fully closed over selector content/currentness, the exact admitted policy-resolution basis, selected-policy content/currentness, declared inputs, and complete accounting; (2) exception governance did not distinguish typed-policy authority, exact record authorization, admission, and application; (3) non-blocking clarification that Exploratory is the source label for numeric level 1 but Posture Scope is not a level; (4) remove the representation-level byte-for-byte phrase. The prior P3 claiming Exploratory was undefined was withdrawn after source clarification.
Deferred or out-of-scope concerns: concrete mappings, thresholds, schemas, Rust types/APIs, registry/storage mechanics, owner crate or service placement, migrations, slice/task decomposition, rollout, external adoption, and implementation order
Project sources: no managed bundle is configured. Read only the complete current artifact and exact companion paragraph below. Excluded source is unavailable and must not be inferred. The bounded source clarification remains: rubric labels 1-5 are Exploratory, Prototype, Product, Production, Hardened; canonical Charter governance has decision authority, required approvals, free-form exception policy, approvers, record location, and minimum fields, but no executable typed exception-policy or exception-record admission mechanism.
Validation evidence: local Markdown links passed; diff whitespace checks passed; the scenario corpus remains exactly 12 rows; PROPOSED and no V1/V2 framing remain intact; remediation anchors for content identity, policy-resolution basis, transitive binding, exception governance, selector self-authorization, level-1 label, unchanged semantic gate truth, and same-output/different-authority identity all passed

Do not reduce the task or replace it with an easier alternative. Preserve the requested scope and project conventions.

For remediation-follow-up, report a blocking finding only when it is reachable through supported behavior, violates an in-scope invariant or accepted remediation objective, meets the material-consequence threshold, and was introduced by the delta, is a regression caused by it, or proves that an accepted root cause remains unfixed. You may inspect unchanged supporting code to establish causality, but unchanged pre-existing behavior is not an in-scope finding merely because you observed it.

After the accepted findings are fixed and no qualifying delta regression remains, stop. Do not enumerate adjacent lexical variants, unsupported dialects, new parser classes, or broader platform behavior unless they prove the same causal mechanism remains unfixed, were introduced or materially worsened by the delta, or review mode is comprehensive-audit. Conservative false negatives are non-blocking in remediation-follow-up unless introduced by the delta or explicitly in scope. Label other observations as deferred, not blocking. Always report a reachable material defect introduced by the delta even when its exact form was not anticipated.

The same causal mechanism means the same parsing state, normalization rule, validation omission, provenance error, authority decision, or equivalent trust transition remains operative. Superficial textual similarity is insufficient.

Return findings first by severity with exact section/text, violated invariant, material consequence, and delta causality. State explicitly when there are no qualifying findings. Then give a direct KEEP, ADJUST, or PIVOT verdict for this remediated candidate.

Complete current artifact:

--- BEGIN current posture-assurance-policy-semantic-decision.md ---
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

--- END current posture-assurance-policy-semantic-decision.md ---

Exact current companion evaluation-context paragraph:

--- BEGIN companion paragraph ---
When posture influenced assurance selection, the evaluation context also fixes the Resolved Posture Snapshot fingerprint; exact Assurance Policy Selector identity, version, semantic-content identity, and currentness; exact admitted policy-resolution basis identity and currentness; exact selected Assurance Policy identity, version, semantic-content identity, and currentness; and Resolved Assurance Requirements identity. The requirements identity transitively binds the admitted exception inputs, every other declared selection input, and complete selection accounting. Those bindings participate in evaluation fingerprinting, currentness, and canonical explanation. Metrics and gates consume the resolved requirements; they never reread or reinterpret mutable posture, reselect requirements, or change metric meaning.
--- END companion paragraph ---
~~~~

### Response

~~~~text
Worked for 3m 33s
Findings

No qualifying P1, P2, P3, or P4 findings.

The remediation-follow-up threshold requires either an accepted root cause that remains operative or a regression introduced or materially worsened by the remediation, with a reachable disagreement over one of the bounded semantic authorities. Neither condition is present in the current artifact or companion paragraph.

Pasted markdown

Accepted-remediation verification
1. Assurance-selection provenance and currentness — fixed

The accepted P2 root cause is closed.

The revised vocabulary now makes the selector’s exact semantic-content identity and currentness distinct from both the policy-resolution basis and the selected policy. Resolved Assurance Requirements transitively binds the snapshot, selector, exact policy-resolution basis, selected policy, admitted exceptions, all other declared inputs, and complete candidate accounting.

Pasted markdown

The selection invariants additionally:

close selector inputs to exact, declared, admitted, currentness-bound inputs;

prohibit classification, mutable posture, free-form Charter prose, gate results, enforcement dispositions, and ambient state from becoming implicit inputs;

require exact accounting over the consulted policy-resolution basis;

require changed selector content, basis, selected-policy content, currentness, or another declared input to change identity or refuse;

require selected, excluded, inapplicable, ambiguous, and refused candidate accounting;

transitively bind all currentness-bearing inputs into the resolved identity.

Pasted markdown

The deterministic-replay scenario now expressly distinguishes a different policy or resolution basis that yields equivalent-looking requirement references, requiring a distinct provenance-bound identity or refusal.

Pasted markdown

The companion evaluation-context paragraph carries the same selector-content, policy-resolution-basis, selected-policy, requirements-identity, exception-input, declared-input, and complete-accounting closure. It therefore does not reopen the original cross-record ambiguity.

Pasted markdown

2. Exception governance, authorization, admission, and application — fixed

The accepted exception-authority P2 root cause is closed.

The revised exception section expressly separates:

Charter governance nomination;

typed exception-policy authority;

authorization of an exact record;

admission of that record;

application of its bounded effect during selection.

Pasted markdown

It also states that Charter prose, approver names, process metadata, record location, and minimum fields are governance inputs rather than executable authority. Assurance selection may neither mint nor approve, broaden, infer, or self-authorize an exception. The acts must remain semantically distinct and independently attributable even when one eventual component hosts multiple responsibilities.

Pasted markdown

The responsibility map assigns each semantic act consistently without selecting an owner crate, service, schema, or API.

Pasted markdown

The discriminating scenario now covers all three previously material failure paths: free-form governance prose, a typed record from an unauthorized actor, and a selector-synthesized or selector-approved record. All refuse.

Pasted markdown

3. Numeric level-1 Exploratory clarity — fixed

The record now defines Exploratory as the current authoring-rubric label for numeric level 1, while expressly denying that rubric labels are posture types or a project-wide scalar.

Pasted markdown

The invariant uses “numeric Posture Level” and explicitly anchors Exploratory to level 1; the scenario no longer describes an “Exploratory scope,” instead describing a supported snapshot that selects a lighter requirement set. This preserves the distinction between a Posture Scope and a numeric dimension level.

Pasted markdown +1

The prior P3 alleging that Exploratory was undefined remains withdrawn.

4. Representation-level guarantee — fixed

The prior “byte-for-byte semantic truth” wording has been removed.

The scenario now requires the exact semantic gate decision and canonical explanation to remain unchanged while allowing separately attributed enforcement dispositions. That is consistent with the normative enforcement section and does not imply an undeclared serialization or byte-comparison contract.

Pasted markdown +1

Delta causality

No accepted causal mechanism remains unfixed, and no P3 or P4 semantic or consistency regression was introduced by the remediation. The edits remain within the proposed semantic-authority boundary and do not introduce implementation authority, schemas, APIs, mappings, owner placement, task decomposition, or rollout claims.

Pasted markdown

Verdict: KEEP

The remediated candidate narrowly and completely resolves the accepted findings. The current posture artifact and companion evaluation-context paragraph are mutually consistent on selection provenance/currentness, selector-input closure, exception authority, level-label semantics, and preservation of semantic gate truth.

No further adjustment is required within this remediation-follow-up boundary.

Sources
~~~~

## Local reconciliation

- Accepted and remediated: the selector and Resolved Assurance Requirements now close transitively over exact selector content/currentness, the admitted policy-resolution basis/currentness, selected-policy content/currentness, admitted exceptions, all declared inputs, and complete accounting.
- Accepted and remediated: Charter governance nomination, typed exception-policy authority, exact record authorization, record admission, and selection-time application are now distinct semantic acts; Charter prose and metadata remain non-executable.
- Repository clarification: the initial P3 claim that Exploratory was undefined was incorrect because the authoring source names level 1 Exploratory. ChatGPT Pro withdrew that finding; the candidate now clarifies the label without treating scope as a level.
- Accepted clarity cleanup: the enforcement scenario preserves exact semantic gate truth without asserting a byte-level representation contract.
- Final advisory verdict: KEEP, with no qualifying P1-P4 findings in the remediation-follow-up boundary.
- No concrete mappings, thresholds, schemas, APIs, owner placement, task decomposition, rollout, or implementation authority were added.
