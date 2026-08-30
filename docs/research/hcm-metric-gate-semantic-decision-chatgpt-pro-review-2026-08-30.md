# HCM metric/gate semantic decision — ChatGPT Pro review

**Date:** 2026-08-30
**Model:** GPT-5.6 Sol, ChatGPT Pro, power 5/5
**Conversation:** https://chatgpt.com/c/6a94ad23-79e8-83ea-bb31-9f91c2841528

> Advisory context only; local repository inspection and validation determine final actions.

This record preserves the complete bounded consultation prompt/response sequence for
the proposed HCM metric/evaluation/gate semantic decision. The product artifact
remains the current source for the candidate semantics.

## Round 1 — independent approach review

### Prompt

~~~text
We are already working on a proposed semantic decision record for a greenfield redesign of a contract evaluation/metric/gate seam.

Task/spec:
Independently review the attached-in-text proposed record. It must narrowly and completely lock only (1) canonical vocabulary and relationships, (2) the responsibility map across source providers, adapters/docks, evidence admission, claim evaluation, named metrics, gate policy, and downstream consumers, and (3) a discriminating scenario corpus. It must not become an implementation plan, schema/API design, migration plan, phase/slice decomposition, adopter choice, parent-promotion policy, posture algorithm, UI design, expression language, or metric marketplace.

Current approach:
The candidate separates the semantic chain into source observation -> untrusted evidence candidate -> Contracts-owned canonical evidence admission -> claim evaluation -> zero or more named metric results -> pure semantic gate decision -> separately owned downstream disposition/mutation. It removes weights and gate effects as intrinsic claim properties; permits weighting only inside a named metric; keeps direct non-compensatory predicates distinct from metric predicates; separates applicability, finding, sufficiency, currentness, consistency, uncertainty, criticality, and decision; and preserves exact provenance/accounting. Repository-derived statement: current V1 puts GateEffect and ScoreWeight on each claim, has one unnamed passed-weight/total-weight score under one ClosedAccounting gate algorithm, and emits a binary pass/blocked gate. Repository-derived statement: current Contracts-owned evidence admission, applicability-before-evidence, immutable content-addressed evidence, source authority/currentness, and fail-closed accounting boundaries are product truth to preserve.

Evidence, blockers, constraints, and validation:
- The artifact is status PROPOSED and must remain non-implementation authority until human approval and later planning.
- The repository is greenfield for this seam; the intended replacement may supersede V1. Do not request a separate V1 patch, compatibility compiler, migration, or dual-run plan.
- Current V1 source and completed Phase-5 documents remain runtime truth until replacement; V1 names may be marked legacy but must not be misrepresented.
- This review is semantic and bounded. Do not expand into implementation design.
- The local authoring review found no P1/P2; Markdown links and exact changed-file scope passed. The repository's GitNexus runner was unavailable in the delegated worktree, which is irrelevant to the semantic review.
- Treat the candidate below as the complete review subject; excluded repository source is unavailable and must not be inferred beyond the explicit repository-derived facts above.

Review questions:
1. Is the vocabulary one coherent domain model, or are terms conflated, missing, or assigned at the wrong abstraction?
2. Does every responsibility boundary identify both the owner and what it cannot decide, without semantic authority leaking into adapters, metrics, gates, or consumers?
3. Are numeric/measurement/weight semantics coherent for heterogeneous external systems without prematurely selecting a schema or transport representation?
4. Are the gate states, direct-vs-metric predicates, explanation/accounting, and downstream disposition separation internally consistent?
5. Does each scenario have a decisive expected outcome, and is the corpus sufficient to expose seam regressions without becoming an implementation test plan?
6. Identify any statement that contradicts the stated repository truth, reopens an excluded product question, or makes an arbitrary implementation choice masquerade as product semantics.
7. Check specifically whether the JSON-safe numeric bound, exact decimal/rational normalization rule, four-state gate decision, all-not-applicable behavior, degradation monotonicity, and the named Weighted Binary Pass Rate are genuinely semantic decisions and mutually coherent.

Do not reduce the task or replace it with an easier alternative. Preserve the requested scope and project conventions.

Return:
- A direct KEEP, ADJUST, or PIVOT verdict.
- Findings first, classified P1 Critical / P2 Required / P3 Optional / P4 Nit. For each, quote only the minimum phrase needed, name the violated semantic invariant, explain the downstream consequence, and propose the smallest bounded correction.
- Explicitly state when there are no P1/P2 findings.
- A compact coverage matrix for vocabulary, responsibilities, numeric semantics, gate semantics, and scenarios.
- A short list of exact edits required before human review; do not draft implementation plans or schemas.
- Flag any recommendation that depends on repository facts not supplied, rather than guessing.

PROPOSED RECORD START

# HCM metric, evaluation, and gate semantic decision

**Status: PROPOSED**

This record fixes the product vocabulary, responsibility boundaries, numeric
meaning, and discriminating scenarios for the Handbook Contract Membrane
metric/evaluation/gate seam. It is a bounded semantic correction after Phase 5
and before Phase-6 consumption. It is not implementation authority until a
human approves it and a later task-level plan separately authorizes delivery.

Current source and the completed [HCM-5.1](slices/HCM-5.1/README.md) and
[HCM-5.2](slices/HCM-5.2/README.md) authorities remain truth for V1. This
record defines the intended replacement semantics; it does not patch V1 or
authorize compatibility or migration work.

## Canonical model and vocabulary

The canonical semantic chain is:

```text
source observation
  -> evidence candidate
  -> Contracts-owned evidence admission
  -> canonical evidence record
  -> applicability and claim evaluation
  -> zero or more named metric results
  -> gate policy
  -> semantic gate decision
  -> separately owned downstream disposition or mutation
```

The following terms are normative. Terms in one row are not synonyms for terms
in another row.

| Term | Fixed meaning |
| --- | --- |
| **Source observation** | A source provider's bounded account of what it observed, including source-native value, unit or scale, subject/snapshot identity, collection state, and provenance. It is not canonical evidence and cannot satisfy a claim by itself. |
| **Evidence candidate** | A dock- or adapter-produced, evidence-shaped submission that binds one or more source observations to an intended evaluation context. It is untrusted input to admission. Do not call it an evidence record or shorten it to “evidence” where that would imply admission. |
| **Canonical evidence record** | The immutable, content-addressed record produced only by Contracts-owned admission after exact identity, provenance, authority, subject, context, unit/scale, normalization, currentness-at-admission, and duplicate checks. “Evidence record” means this admitted object. |
| **Applicability result** | The Contracts-owned determination, made before evidence selection, that a claim is `applicable`, `not_applicable`, or `undetermined`, with an exact basis. `not_applicable` is positive semantic proof, not missing data. |
| **Claim evaluation** | The complete result for exactly one expected claim in one evaluation context: its applicability result and, when applicable, an established finding or an indeterminate determination, plus the independent evidence-assessment dimensions and complete accounting. |
| **Finding** | The semantic satisfaction result established by a claim evaluator: `satisfied`, `unsatisfied`, or a value on a claim-declared named grade/measurement scale. There is no universal partial-satisfaction scalar. |
| **Metric definition** | An immutable, named and versioned algorithm invocation over a declared population of compatible claim evaluations, with explicit input contract, scale, unit, aggregation, parameters, and missing/indeterminate-input policy. |
| **Metric result** | The exact output of one metric definition: `computed` with a value on its named scale, `not_applicable`, or `not_computable`, plus included/excluded inputs and a reproducible calculation trace. There is no unqualified “aggregate score.” |
| **Gate policy** | An immutable semantic decision rule over direct claim predicates and named metric predicates, including condition precedence and treatment of indeterminacy. |
| **Semantic gate decision** | The non-mutating result `pass`, `fail`, `indeterminate`, or `not_applicable`, with every condition result and decisive reason. It answers the policy's bounded semantic question only. |
| **Downstream disposition** | A separately owned enforcement, workflow, posture, or promotion interpretation of a semantic gate decision, such as allow, block, hold, or no effect. A mutation, if any, remains another authority-controlled operation. |

An **evaluation context** fixes one contract revision, subject snapshot,
evaluation time, evidence-set identity, claim-evaluator versions, metric
definitions, and gate-policy revision. Results that do not share that closure
cannot be combined.

## Independent dimensions

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

## Measurement and metric semantics

Every metric definition must state all of the following; omission makes the
definition invalid rather than supplying a default:

1. a stable metric name and algorithm version;
2. the exact eligible claim-evaluation population and compatibility rules;
3. the input and output scale, including bounds, direction, and permitted
   operations;
4. the quantity dimension and canonical unit where quantities are involved;
5. the exact aggregation algorithm and parameters;
6. handling for `not_applicable`, indeterminate, insufficient, stale,
   conflicting, uncertain, missing, or incompatible inputs; and
7. complete included/excluded-input and calculation accounting.

Externally stable numeric values are exact. Finite decimals retain their exact
coefficient and scale; ratios retain exact numerator and positive denominator;
quantities retain exact value and unit; named categorical or ordinal scales
retain their scale identity and direction. No producer, adapter, metric, or
transport may implicitly coerce an IEEE-754 value, round a boundary, infer a
unit, average an ordinal scale, or collapse a quantity to a unitless number.
Exact conversions must be named, versioned, dimension-compatible, and visible
in the trace.

Public signed integer coefficients are bounded to
`-9,007,199,254,740,991..=9,007,199,254,740,991`. Counts, normalized weights,
nonnegative metric numerators/denominators, and their published sums are bounded
to `0..=9,007,199,254,740,991`; a semantic domain that requires a positive
value excludes zero. Arithmetic may use wider checked intermediates, but an
out-of-bound externally visible result refuses instead of clamping, rounding,
wrapping, or changing representation silently. Threshold comparison is exact
and declares whether equality passes.

A weight exists only as a parameter inside one named metric definition. It is
not intrinsic to a claim and never means obligation, criticality, severity,
confidence, producer trust, or gate effect. Relative rational or finite-decimal
weights are normalized exactly to the smallest positive integer vector that
preserves their ratios; normalization and total-bound failure reject the metric
definition.

Weighted computation is admitted only as the named **Weighted Binary Pass
Rate** metric. Its semantic algorithm is fixed here; its schema and algorithm
identity/version representation remain for later planning:

```text
population  = the definition's exact claim population after proven N/A removal
requirement = every remaining member has an established binary finding
numerator   = sum(metric-local weight for satisfied population members)
denominator = sum(metric-local weight for all population members)
value       = numerator / denominator, exactly
```

It rejects graded or incompatible findings. A missing, indeterminate, stale,
conflicting, or incomplete required input makes the result `not_computable`
under its fixed missing-data policy; a legitimately empty all-not-applicable
population makes it `not_applicable`. Neither case yields `1/1` or 100%.

## Responsibility map

| Layer | Owns | Must not decide |
| --- | --- | --- |
| Validators and source providers | Bounded source-specific execution, source observation, source snapshot identity, native values/units/scales, and collection diagnostics. | Applicability, claim findings, canonical admission, metric weights/values, gate policy, or downstream action. |
| Docks and adapters | Protocol execution/framing, exact translation, preservation of source semantics and lexemes, candidate construction, and import diagnostics. | Canonical evidence, invented units or scale direction, silent normalization, claim meaning, applicability, metrics, or gates. |
| Contracts evidence admission | Exact candidate/source/run/subject binding; authority and provenance validation; canonical normalization; unit/scale identity and permitted exact conversion; idempotent duplicate identity; immutable canonical evidence. | Claim satisfaction, criticality, cross-claim aggregation, gate outcome, or workflow action. |
| Claim evaluation | Applicability; complete evidence selection/accounting; sufficiency, currentness, consistency, uncertainty treatment; semantic interpretation; and one finding or indeterminate determination per expected claim. | Cross-claim metric aggregation, global thresholds, gate precedence, enforcement, posture, promotion, or mutation. |
| Metric computation | Pure exact derivation from compatible claim evaluations under one named definition, including population and calculation accounting. | Raw-source/evidence selection, applicability changes, finding reinterpretation, direct blockers, final decision, or disposition. |
| Gate policy | Direct non-compensatory claim predicates, named metric predicates, completeness/currentness predicates, precedence, and the semantic gate decision. | Raw evidence interpretation, unit/scale conversion, changing metric semantics, canonical admission, or any workflow/posture/promotion mutation. |
| Downstream consumers | Their own versioned mapping from the exact semantic decision to enforcement, posture, workflow, or promotion disposition; any separately authorized mutation with fresh authority/currentness checks. | Rewriting the gate decision, treating a metric as the decision, bypassing decisive provenance, or transferring mutation authority into Contracts, a dock, SDK, or transport. |

## Gate and explanation invariants

- Direct claim predicates and named metric predicates are different policy
  constructs. A direct predicate is non-compensatory and is never encoded only
  as a large weight.
- Metric results do not compensate for one another unless a future named gate
  policy explicitly declares a compatible composite rule. There is no ambient
  averaging across metrics.
- A gate decision is pure derived semantic truth. It persists or mutates no
  lifecycle, work unit, posture, parent, promotion, release, or deployment
  state and grants no authority to do so.
- Complete claim accounting is mandatory: every claim in the fixed evaluation
  context appears exactly once as applicable/evaluated, not applicable, or
  undetermined. Omission is invalid, not `not_applicable`.
- Evidence degradation cannot produce `pass` from `fail` or `indeterminate`,
  nor preserve a prior pass by hiding changed accounting. Without a genuine
  applicability change, removing or staling evidence, losing provenance,
  reducing sufficiency, introducing conflict, or widening uncertainty cannot
  make the downstream disposition less restrictive under the same policy.
- The explanation is canonical decision data, not reconstructed prose. It
  preserves expected claim inventory; applicability basis; every included,
  excluded, disqualified, stale, conflicting, and duplicate evidence binding;
  finding and assessment dimensions; each metric definition, population,
  value/state, conversion, and calculation; every gate condition and
  precedence step; and the decisive reason/accounting chain.

## Discriminating scenario corpus

Each scenario assumes exact identity/provenance and the stated immutable policy
unless the row identifies their failure.

| Scenario and discriminating inputs | Responsible layers | Expected semantic outcome | Invariant proved |
| --- | --- | --- | --- |
| **Hard failure with high aggregate.** One direct required security claim is established unsatisfied; Weighted Binary Pass Rate is `99/100`. | Claim evaluator; metric; gate policy. | Metric remains `99/100`; direct predicate fails; gate decision is `fail`. | A compensatory aggregate cannot override a direct predicate. |
| **Missing versus not applicable.** Claim A is applicable but has no admitted record; Claim B has a proved false applicability rule. Policy requires every applicable required claim to be determinate. | Applicability and claim evaluators; gate. | A is indeterminate with insufficient coverage; B is `not_applicable`; metric requiring A is `not_computable`; gate is `indeterminate`, not pass. | Absence, omission, and N/A are distinct. |
| **Stale evidence.** The only satisfying record exceeds its freshness policy at the explicit evaluation time. | Claim evaluator; metric; gate. | Currentness is stale, finding is indeterminate, dependent metric is `not_computable`, and gate is `indeterminate`. | Historical satisfaction is not current satisfaction. |
| **Conflicting evidence.** Two eligible authorized records establish opposite findings and no resolver is named. | Claim evaluator. | Consistency is conflicting; claim determination and dependent metric are indeterminate/not computable; gate is `indeterminate`. | Neither adapter nor metric may cherry-pick. |
| **Duplicate evidence.** The same candidate identity and bytes are submitted twice. | Admission; claim evaluator. | Admission is idempotent to one canonical evidence identity; accounting, finding, metrics, and gate bytes are unchanged. A same-identity/different-bytes submission refuses. | Duplication cannot increase weight or confidence. |
| **Flaky observation.** Ten distinct eligible runs contain nine satisfied and one unsatisfied observation; the claim names stability consensus, not a sampling model. | Claim evaluator. | Consistency is unstable/conflicting and the claim is indeterminate; no generic “90% satisfied” value is produced. | Repetition needs a named sampling model before scalar aggregation. |
| **Quantitative unit normalization.** Authorized observations report `1 s` and `1000 ms` for the same latency quantity and policy permits the exact unit conversion. | Admission; claim evaluator; metric. | Both normalize to the same exact canonical quantity and produce the same finding and metric value with source units retained in provenance. | Units are semantic and exact, not labels. |
| **Graded external scale.** An imported grade is `1` on a versioned ordinal scale where lower is better; a named claim evaluator maps it to a declared grade. | Adapter; admission; claim evaluator; compatible metric. | Direction is honored from the exact scale definition; a binary metric rejects the grade; only a metric naming that scale/mapping may consume it. | External numbers do not imply direction, binary truth, or weights. |
| **Decimal weights and exact bounds.** Source weights `0.25`, `1.50`, `2.25` are exact; the named metric uses relative weights and an inclusive exact threshold. | Adapter; metric-definition admission; metric. | Weights normalize to `1, 6, 9` with a receipt; threshold equality follows the declared inclusive comparator. Any coefficient/vector/sum above `9,007,199,254,740,991` rejects the definition before evaluation. | No floating coercion, rounding, or overflow fallback. |
| **All not applicable.** Every selected claim is proved `not_applicable`; the stated downstream policy maps gate `not_applicable` to `no_effect`. | Applicability evaluator; metric; gate; downstream consumer. | Metric and gate are both `not_applicable`, never `pass`; downstream disposition is `no_effect`. | Empty populations do not pass vacuously. |
| **Degradation monotonicity.** A passing record is removed from the selected evidence-set identity, becomes stale, loses provenance, or gains an unresolved contradiction while applicability is unchanged. | Claim evaluator; metric; gate; downstream consumer. | Re-evaluation changes to indeterminate/fail as dictated by the affected dimension and cannot yield pass or a less restrictive same-policy disposition. | Evidence degradation cannot improve the decision. |
| **Mixed population closure.** Candidate claim evaluations differ in subject snapshot, contract revision, evaluator version, or evaluation time. | Metric computation; gate. | The metric is `not_computable` with every incompatible input accounted; the dependent gate is `indeterminate`. | Metrics cannot span mixed semantics or ambient time. |
| **Multiple named metrics.** Functional pass rate meets its threshold; a separate security-severity metric violates its threshold. | Two metrics; gate policy. | Results remain separate; the security metric predicate is decisive and the gate is `fail`. No hidden cross-metric average exists. | One metric cannot silently compensate for another. |
| **Uncertain boundary.** A latency interval straddles the claim budget and no alternate evaluator is named. | Claim evaluator; metric; gate. | Uncertainty is preserved; the finding is indeterminate, metric is `not_computable`, and gate is `indeterminate`. | Midpoints or favorable endpoints cannot be selected silently. |

## V1 reconciliation

These names describe current implementation truth but are legacy at the new
semantic seam:

| V1 name | Reconciliation |
| --- | --- |
| `ContractClaimV1` (sometimes described conceptually as `ClaimDefinitionV1`) | Legacy claim shape. Its normative claim identity/evidence requirements remain relevant, but claim-attached gate effect and score weight do not survive as intrinsic claim properties. |
| `EvidenceCandidateV1` | Remains an evidence candidate only; it is never admitted evidence. |
| `EvidenceRecordV1` | Aligns with canonical evidence record, subject to the expanded observation/unit/scale semantics selected by later design. |
| `ApplicabilityOutcomeV1::Blocked` | Legacy conflation. The canonical applicability term is `undetermined`; fail-closed gate/downstream behavior is decided separately. |
| `ClaimVerdictV1` / `ClaimVerdictDecisionV1` | Legacy compound result. Canonical claim evaluation separates applicability, established finding versus indeterminate determination, and assessment dimensions. |
| `GateEffectV1` | Legacy claim-attached policy. Non-compensatory claim predicates belong to a gate policy; criticality/severity remains separate. |
| `ScoreWeightV1` | Legacy claim-attached parameter. Weights belong only to one named metric definition. |
| `ScoreResultV1` | Legacy unnamed single-ratio result. It is not the universal aggregate. The replacement Weighted Binary Pass Rate has the semantics fixed above; this record does not claim byte or behavioral compatibility with the V1 deficit-counting seam. |
| `GatePolicyV1`, `GateResultV1`, and `GateDecisionV1` | Legacy one-algorithm/two-decision seam. The canonical gate policy consumes direct predicates plus named metric results and emits the four-state semantic gate decision without mutation. |

This reconciliation permits the intended replacement to supersede V1. It does
not select a V1 patch, compatibility compiler, dual run, migration mechanism,
or preserved wire/API shape.

## Settled versus deferred

This record settles the vocabulary and relationships, independent dimensions,
exact measurement rules and bounds, weight ownership, responsibility map, gate
purity and precedence, complete explanation/accounting requirement, and every
scenario outcome above. Later work may not reopen those decisions merely by
choosing a representation.

Deliberately deferred to human-approved task-level planning are schema and Rust
type shapes, API and operation shape, concrete algorithm catalog beyond the
named Weighted Binary Pass Rate semantics, expression language, persistence
layout, registry/versioning mechanics, V1 migration or compatibility, delivery
packets, acceptance plan, and rollout. Also out of scope are external adopter
selection, parent-promotion policy, posture algorithms, UI, a metric
marketplace, and a universal confidence score.

Human approval of this proposed record would make it semantic input to later
planning only. It would not by itself authorize implementation, change a phase
or slice status, or establish any delivery outcome.


PROPOSED RECORD END
~~~

### Response

~~~text
Verdict: ADJUST

The proposed record has the right domain shape and does not require a pivot. The source-observation → admission → claim evaluation → named metric → semantic gate → downstream-disposition chain is coherent, and the separation of applicability, finding, assessment dimensions, metrics, decisions, and mutations is directionally correct.

There are no P1 Critical findings. There are six P2 Required findings. All six arise from the candidate’s own text; none depends on unavailable repository material. This review treats the supplied candidate and repository-derived facts as the complete review boundary.


Findings
P2 Required — Metric-definition authority is missing, and the numeric scenario gives the metric layer a gate threshold

Minimum phrases: “metric-definition admission”; “an inclusive exact threshold.”

Violated invariant: Every semantic decision must have an identified owner, and a metric must compute its named value without deciding the gate predicate applied to that value.

The record declares that an incomplete metric definition is invalid, but the responsibility map contains only metric computation, not an authority that admits or rejects immutable metric definitions. The scenario later invokes a “metric-definition admission” layer that does not exist in that map.


The same scenario says that the named metric uses an inclusive threshold. That contradicts the responsibility map: the metric owns exact derivation, while the gate policy owns named metric predicates and therefore the threshold/comparator used to decide whether a metric condition is met. Calling the inputs “source weights” and including an adapter among the responsible layers also conflicts with the rule that source providers and adapters cannot decide metric weights or metrics.


Downstream consequence: Different implementations could allow an adapter, metric runtime, or gate policy to redefine population compatibility, weight normalization, bounds, or missing-input behavior. A metric could also begin emitting pass/fail semantics rather than its exact ratio, collapsing the metric/gate seam the record is intended to protect.

Smallest bounded correction:

Rename the responsibility row to Named metric definition and computation, or add a separate Metric-definition authority/admission row.

Assign that owner validation of the stable name/version, population, compatible scales, units, parameters, weight validity, bounds, aggregation, and missing-input policy.

Rewrite the decimal-weight scenario so that the metric definition declares the positive weights, the metric computes a ratio, and the gate policy separately declares the exact threshold and inclusive comparator.

Remove the adapter from the responsible layers unless its only role is preserving an externally supplied proposal’s exact lexemes without semantic authority.

Whether metric-definition authority is folded into the existing metric layer or represented as a separate HCM authority is a product choice not established by the supplied repository facts.

P2 Required — The gate-condition taxonomy and the meaning of not_applicable are incomplete

Minimum phrases: “direct claim predicates”; “completeness/currentness predicates.”

Violated invariant: Canonical categories must be mutually distinguishable and sufficient to explain every gate state.

The vocabulary defines gate policy as operating over only direct claim predicates and named metric predicates. The responsibility map then introduces completeness and currentness predicates without saying whether those are direct claim predicates or a third policy construct.


The four decision names are enumerated, but the record never gives a general semantic meaning for gate-level not_applicable. The all-not-applicable scenario asserts that result, but it does not state whether that is a universal rule, a policy-declared gate-applicability rule, or merely the behavior of that scenario’s unnamed policy.


Downstream consequence: One implementation could model currentness as a direct non-compensatory claim condition, another as a global gate condition, and another as hidden treatment of metric non-computability. They could also disagree about whether an empty policy population is pass, not_applicable, or indeterminate, while each claims conformance.

Smallest bounded correction:

Define direct claim-evaluation predicate as a non-compensatory predicate over exactly one claim evaluation, including its applicability, determination, finding, sufficiency, currentness, consistency, or uncertainty dimensions. Keep a named metric-result predicate as the other construct.

Then define the four states semantically:

pass: the policy applies and establishes its bounded proposition.

fail: the policy applies and establishes its negation or a policy-declared fail-closed violation.

indeterminate: the policy applies but cannot establish pass or fail under its declared indeterminacy treatment.

not_applicable: positive semantic proof establishes that the policy’s governed population or question does not apply; omission, missing inputs, or inability to compute never produce this state.

That is a state model, not an expression language.

P2 Required — The record contradicts its own distinction between a finding, an indeterminate determination, and undetermined applicability

Minimum phrases: “finding is indeterminate”; “applicable/evaluated … or undetermined.”

Violated invariant: undetermined applicability, an applicable-but-indeterminate claim determination, and an established finding are separate semantic facts.

The canonical vocabulary correctly says that applicability may be undetermined, while an applicable claim has either an established finding or an indeterminate determination. A finding itself is satisfied, unsatisfied, or a value on a declared scale.


Later, complete accounting is reduced to “applicable/evaluated, not applicable, or undetermined,” which does not explicitly account for an applicable claim whose finding cannot be established. The stale and uncertain scenarios then say that the “finding is indeterminate,” directly contradicting the fixed definition of finding.


Downstream consequence: Metric eligibility can once again depend on an ambiguous compound verdict. Implementations may serialize or interpret “indeterminate” as a finding value, an applicability state, or absence of a finding, recreating the V1-style conflation the record is intended to remove.

Smallest bounded correction:

Replace the accounting invariant with two explicit stages:

Every expected claim appears exactly once with one applicability result. Every applicable claim then appears exactly once with either one established finding or one indeterminate determination. An applicability result of undetermined establishes no finding. Omission is invalid.

In the stale and uncertain scenarios, replace “finding is indeterminate” with:

no finding is established; the claim determination is indeterminate.

Reserve undetermined exclusively for applicability.

P2 Required — Several scenarios are not decisive, and some assert representation-level behavior

Minimum phrases: “gate is indeterminate”; “as dictated”; “gate bytes are unchanged.”

Violated invariant: Every discriminating scenario must contain enough semantic inputs to force exactly one expected outcome, without silently selecting a schema, encoding, receipt type, or execution protocol.

Several rows produce a gate result without stating the gate policy’s treatment of indeterminate claims or non-computable metrics:

stale evidence;

conflicting evidence;

mixed population closure;

uncertain boundary.

The conflicting-evidence row names only the claim evaluator as responsible even though its expected outcome includes a metric and gate result.


Other rows remain non-decisive:

“stability consensus” does not say whether unanimity, majority, or another deterministic resolver is intended;

the decimal-weight row supplies no binary findings, computed ratio, exact threshold, or single-condition gate policy from which a threshold-equality outcome can be derived;

the degradation row combines several materially different changes and allows either “indeterminate/fail as dictated,” rather than fixing an outcome.


The duplicate scenario’s assertion that “gate bytes are unchanged” selects serialization-level equivalence even though schema and identity representation are expressly deferred. Likewise, “with a receipt” introduces an undefined artifact type where the established vocabulary already provides calculation trace and accounting.


Downstream consequence: Two conforming gate policies could legitimately return different decisions for the same scenario. A later implementation could also be judged non-conforming merely because it uses a different canonical encoding or does not create an object named “receipt.”

Smallest bounded correction:

For every row that reaches a gate result, state:

the exact required condition or metric predicate;

whether a decisive failure is present;

how that policy treats an indeterminate claim or not_computable metric;

the relevant precedence.

Also:

Define the flaky scenario as an explicit unanimity requirement, or name another exact consensus rule.

Give the weight scenario fixed binary findings, an exact computed ratio, and a separately gate-owned threshold equal to that ratio.

Split the degradation row into fixed subcases or select one exact degradation with one expected result.

Add metric and gate layers to rows whose outcome uses them.

Replace “gate bytes are unchanged” with “the semantic decision, canonical identities, and accounting are unchanged.”

Replace “receipt” with “normalization and calculation trace.”

These remain semantic scenarios; no test harness or API shape is required.

P2 Required — “Only” one weighted computation contradicts the explicitly deferred metric catalog

Minimum phrase: “Weighted computation is admitted only.”

Violated invariant: This record may define the semantics of the named Weighted Binary Pass Rate without accidentally deciding the entire future metric catalog.

The candidate says weighted computation is admitted only as Weighted Binary Pass Rate. It later says the concrete algorithm catalog beyond that metric is deliberately deferred. Those propositions conflict: the first permanently prohibits another semantically approved weighted metric, while the second reserves that question for later semantic work.


Downstream consequence: A later compatible weighted metric—such as a dimensionally valid weighted quantity aggregation—would either violate this settled record or require reopening a decision that the record says later planning may not reopen merely through representation choices.

Smallest bounded correction:

Replace the exclusivity sentence with:

The only weighted metric semantics defined and authorized by this record are the named Weighted Binary Pass Rate semantics below. No other weighted computation is implied or authorized; any future weighted metric requires a separate semantic decision.

Also change “inside one named metric definition” to “inside a particular named metric definition,” and explicitly require each Weighted Binary Pass Rate weight to be a strictly positive exact rational or finite decimal.

The algorithm itself can remain unchanged.

P2 Required — Degradation monotonicity is overbroad and improperly constrains downstream disposition

Minimum phrase: “less restrictive.”

Violated invariant: HCM owns the semantic decision; downstream consumers own the interpretation of that decision. A monotonicity rule must also define the information ordering over which it claims monotonic behavior.

The current invariant says evidence removal, staleness, provenance loss, reduced sufficiency, conflict, or widened uncertainty cannot make downstream disposition less restrictive. But the record deliberately gives downstream consumers ownership of their versioned mapping, and it defines no universal ordering of allow, block, hold, no_effect, posture states, workflow states, or promotion states.


“Removing evidence” is also not intrinsically degradation. Removing an ineligible duplicate, irrelevant excluded input, or adverse record later proven unauthorized need not worsen the semantic result. The scenario similarly combines record removal, staleness, provenance change, and newly introduced contradiction as though they were one ordered operation.


Downstream consequence: The HCM record would silently choose a consumer-policy ordering that it explicitly says belongs elsewhere. The monotonicity statement could also reject valid re-evaluations because it has not distinguished loss of eligible decision-supporting information from correction or removal of invalid information.

Smallest bounded correction:

Define degradation narrowly as a change that, while subject, applicability, policy, and all other eligible information remain fixed:

removes or weakens eligible information supporting the previous decision; or

adds unresolved adverse, conflicting, stale, or uncertain information.

Then state only the gate-level invariant:

Such degradation cannot change fail or indeterminate to pass. When it destroys the sole valid basis for a prior pass, the re-evaluated decision cannot remain pass. The changed accounting must be visible.

Delete the universal “less restrictive downstream disposition” clause. A downstream consumer must apply its unchanged mapping to the new semantic decision, but whether that mapping is disposition-monotone belongs to that consumer’s separately owned policy.

P3 Optional findings
P3 Optional — Distinguish admission-time source validity from evaluation-time freshness

“Currentness-at-admission” and evaluation-time current/stale/unknown are both valid concepts, but the record should say expressly that the former is an immutable historical admission fact and the latter is recomputed for the evaluation time under the claim’s freshness policy. That would prevent admission from appearing to make a durable claim-currentness decision.


P3 Optional — A metric definition should not be called an “algorithm invocation”

A definition is more precisely an immutable named/versioned configured rule. The invocation occurs when that definition is applied in an evaluation context to produce a metric result. Changing “algorithm invocation” to “configured algorithm definition” would keep definition identity separate from a particular evaluation.


P3 Optional — State the semantic rationale for the integer bound and remove internal-arithmetic commentary

The ±9,007,199,254,740,991 range can legitimately be a semantic public-domain limit because acceptance or refusal is externally observable. It is mutually compatible with exact decimals, exact ratios, and refusal rather than rounding. However, the record should call it a deliberate cross-runtime binary64-exact interoperability limit rather than implying that JSON itself defines that range. The sentence allowing “wider checked intermediates” is implementation commentary and can be removed; only the externally visible refusal behavior belongs here.


P3 Optional — Add one admission-authority failure scenario

The corpus exercises duplicate handling, units, and scale preservation, but one compact scenario in which an unauthorized source, mismatched subject, or invented adapter unit is refused before claim evaluation would directly protect the supplied repository truth that canonical evidence admission owns authority, subject, provenance, and normalization.


There are no P4 Nit findings worth recording.

Coverage matrix
Area	Assessment	Coverage and correction
Vocabulary	ADJUST	The principal entities and chain form one coherent model. The required correction is to keep applicability undetermined, claim determination indeterminate, and established findings distinct, and to define direct claim-evaluation predicates precisely.



Responsibilities	ADJUST	Source, adapter, evidence admission, claim evaluation, metric computation, gate, and consumer prohibitions are mostly strong. Metric-definition authority is missing, and the decimal scenario assigns threshold/weight authority to the wrong layers.



Numeric semantics	KEEP WITH BOUNDED EDITS	Exact decimals, ratios, quantities, named scales, exact conversions, primitive integer-vector weight normalization, and refusal instead of rounding are mutually coherent. The 0.25, 1.50, 2.25 → 1, 6, 9 normalization is correct. Threshold ownership must move to gate policy, strict weight positivity should be explicit, and the public-bound rationale should be stated.



Gate semantics	ADJUST	Direct non-compensation, metric separation, purity, precedence accounting, and four categorical decisions are appropriate semantic choices. The direct-condition taxonomy, general meaning of gate not_applicable, and degradation scope need correction.



Scenarios	ADJUST	The corpus covers the correct risk families—hard blockers, missing/N/A, freshness, conflict, duplicates, flakiness, units, external scales, exact weights, empty populations, mixed closure, multiple metrics, and uncertainty—but several rows lack enough policy facts to force their stated result and two rows introduce representation terminology.


Explicit assessment of the six called-out decisions
Decision	Review result
JSON-safe integer bound	Genuinely semantic if deliberately adopted as the public interoperable numeric domain. Refusal at the boundary is externally observable and therefore semantic; it need not select a schema. Add the interoperability rationale and remove the internal-arithmetic aside.
Exact decimal/rational normalization	Coherent and semantic. Primitive positive integer-vector normalization preserves ratios exactly, and the example normalizes correctly to 1, 6, 9. Explicitly reject zero and negative weights.
Four-state gate decision	Appropriate semantic choice, incompletely defined. Keep all four states, but define gate applicability and the distinction between a policy that does not apply and a policy that applies but cannot decide.
All-not-applicable behavior	Coherent. Returning metric and gate not_applicable, rather than a vacuous 1/1 or pass, is a legitimate semantic decision. State it as a general gate-applicability rule for the relevant policy shape, not only as a scenario assertion.
Degradation monotonicity	Semantic in a narrowed gate-level form. It is not coherent as a universal downstream-disposition rule, and “degradation” requires a defined information ordering.
Weighted Binary Pass Rate	The algorithm is coherent and properly semantic. Keep its exact population, positive weights, numerator, denominator, not_computable, and all-N/A behavior. Move its threshold to gate policy and change “only weighted computation” to “only weighted metric defined by this record.”
Exact edits required before human review

Add or explicitly identify the authority that validates and admits named metric definitions; keep metric computation pure.

Move threshold/comparator ownership in the decimal scenario from the metric to gate policy, and remove source/adapter ownership of weights.

Define direct claim-evaluation predicates and give semantic meanings for all four gate states, especially not_applicable.

Rewrite complete claim accounting and every “finding is indeterminate” occurrence to preserve the applicability/finding/determination distinctions.

Make every scenario decisive by adding the necessary policy, resolver, population, precedence, and exact numeric facts; correct responsible-layer columns and remove “gate bytes” and “receipt.”

Rephrase Weighted Binary Pass Rate exclusivity so the record defines that metric without permanently deciding the deferred metric catalog.

Narrow degradation to a defined loss or weakening of eligible decision-supporting information, retain only the gate-level monotonicity guarantee, and remove the universal downstream restrictiveness requirement.

No required recommendation above assumes undisclosed repository behavior. I did not verify the linked Phase-5 documents, actual V1 type definitions, or any omitted implementation. The only unresolved owner choice is whether metric-definition authority should be part of the existing named-metric layer or a separate HCM authority; the supplied record must select one, but the supplied facts do not justify guessing which organizational shape already exists.
~~~

## Round 1 clarification — repository-grounded owner boundary

### Prompt

~~~text
Repository truth resolves the one owner question you correctly marked as unavailable.

Bounded current-source facts:
- The current HCM architecture selects the existing Contracts domain owner; no new semantic-owner crate is selected.
- Current authority says Contracts owns evaluation-run identity, freshness/stability/gate/local policies, verdicts, gate composition, and exact policy identity.
- Freshness and gate policy instances are contract-selected immutable records. Contracts implements their schemas, registries, exact resolution, and evaluation semantics; same-ref changed bytes and unregistered/unknown/substituted policies refuse.
- The live Contracts policy registry registers exact freshness and gate policy bindings and resolves only an exact ref/fingerprint match.
- Current cross-cutting authority says Contracts is the sole evidence/verdict/gate/local-closeout owner and that definition, evidence, policies, verdict, and gate identities are exact and content-addressed.
- Engine supplies only generic facts/authority/currentness; SDK and transports project owner truth and may not recompute policy or metric decisions.

Proposed bounded resolution:
Add a distinct responsibility row named "Contracts-owned named metric-definition authority." It owns canonical construction/re-admission and exact resolution of immutable named metric definitions; validation of name/version, eligible population, compatible scales/units, algorithm identity, parameters, strictly positive exact weights and numeric bounds, aggregation, and missing/indeterminate-input policy. It must not select evidence, reinterpret findings, evaluate the metric, apply a threshold, decide a gate, or mutate downstream state.
Keep a separate "Metric computation" row as a pure derivation from an exact admitted definition and compatible claim evaluations.
This is a responsibility distinction inside the existing Contracts owner, not a new crate, registry implementation, API operation, or delivery decision.

Please answer only this bounded clarification:
1. Does this repository-grounded resolution fully close your first P2 without leaking implementation design into the semantic record?
2. Should the semantic artifact use "canonical construction/re-admission and exact resolution" instead of generic "admission" to avoid implying a public mutable ingest operation?
3. Give the exact minimal wording adjustment if the proposed resolution still conflates definition authority and evaluation.
Do not reopen the other findings or expand into implementation/schema planning.
~~~

### Response

~~~text
Yes—with one bounded condition. The proposed row fully closes the missing-owner portion of the first P2 and does so at the semantic level. It assigns definition identity and validity to the already-selected Contracts owner, while expressly excluding runtime metric evaluation, threshold application, gate decision, and mutation. That preserves the record’s intended chain and its existing separation between definition requirements, pure metric computation, and gate policy.


It does not leak implementation design because it does not select a crate, registry representation, API operation, persistence layout, or resolution mechanism. It fixes only semantic ownership, exact-identity behavior, validation responsibility, and prohibited decisions.

To close the entire first P2, the decimal-weight scenario must also reflect that separation: weights are parameters of the exactly resolved metric definition, while the threshold and inclusive comparator belong to gate policy. The adapter should not appear as a semantic owner of either.

Prefer:

canonical establishment, exact re-admission validation, and exact resolution

over either generic “admission” or the slash form “construction/re-admission.”

“Admission” alone can suggest a public mutable ingest operation. “Re-admission” alone can also be misread as creating a replacement definition. Exact re-admission validation instead communicates the required semantic behavior: an already identified immutable definition may be presented again only if its exact identity and content agree; changed bytes under the same identity refuse. “Canonical establishment” is slightly less implementation-shaped than “canonical construction.”

The proposed wording does not materially conflate authority and evaluation, but “eligible population” should be changed to “declared eligible-population rule and compatibility contract.” Otherwise, it can sound as though definition authority selects the actual run inputs.

The minimal two-row wording is:

Contracts-owned named metric-definition authority — Owns canonical establishment, exact re-admission validation, and exact resolution of immutable named metric definitions; validation of exact name/version, the declared eligible-population rule and compatibility contract, input/output scales and units, algorithm identity, parameters, strictly positive exact weights and numeric bounds, aggregation semantics, and missing/indeterminate-input policy. Must not select evidence, reinterpret or establish findings, determine the actual evaluation-run population, compute a metric result, apply a threshold or comparator, decide a gate, or mutate downstream state.

Metric computation — Owns pure exact application of one exactly resolved named metric definition to compatible claim evaluations in one closed evaluation context, including derivation and accounting of the actual included and excluded population and the calculation result. Must not select raw evidence, change applicability or findings, change definition semantics, apply a gate threshold, decide the gate, or determine downstream disposition.

That wording cleanly separates declaring and resolving the population rule from deriving the actual run population and result.
~~~

## Round 2 — remediation follow-up

### Prompt

~~~text
Review mode: remediation-follow-up
Review context: explicitly requested continuity in this same conversation
Review boundary: baseline candidate reviewed in your first answer, plus the complete revised candidate below
Review target: only the remediation of your six accepted P2 findings and any material regression introduced by that remediation

Accepted prior findings:
1. Metric-definition authority missing and threshold assigned to the wrong layer.
2. Gate-condition taxonomy and gate not_applicable meaning incomplete.
3. Applicability undetermined, claim determination indeterminate, and established finding conflated.
4. Scenarios not decisive and representation terms leaked in.
5. Weighted Binary Pass Rate exclusivity contradicted deferred metric catalog.
6. Degradation monotonicity overbroad and improperly constrained downstream disposition.

Applied remediation summary:
- Added the repository-grounded Contracts-owned metric-definition authority and kept exact runtime metric computation separate, using the wording you clarified.
- Moved weights into the metric definition and threshold/comparator into gate policy.
- Defined direct claim-evaluation predicates, named metric-result predicates, and all four gate-decision meanings.
- Rewrote complete accounting and all affected scenarios to distinguish applicability, established finding, and indeterminate claim determination.
- Made scenarios decisive with exact policy treatment/precedence; removed bytes/receipt assertions; added an unauthorized-source scenario.
- Rephrased Weighted Binary Pass Rate as the only weighted semantics authorized by this record, while reserving future weighted metrics to separate semantic decisions.
- Narrowed degradation to loss/weakening of eligible support or addition of unresolved adverse information, retained only gate-level monotonicity, and returned downstream ordering to consumer policy.
- Also adopted your bounded P3 clarifications for admission-time validity vs evaluation-time freshness, configured definition vs invocation, and the cross-runtime rationale for the public numeric bound.

In-scope invariants:
The record must narrowly and completely lock vocabulary, responsibility boundaries, numeric/measurement semantics, gate semantics, and a discriminating scenario corpus. It remains PROPOSED and non-implementation authority. It must not become a schema/API, implementation, migration, phase/slice, expression-language, adopter, promotion, posture, or UI plan.

Required material consequence for a blocking finding:
A remaining accepted root cause, an internal contradiction that permits materially different conforming semantics, an ownership/authority leak, a non-decisive required scenario, or a remediation-introduced regression against the bounded semantic outcome.

Blocking threshold:
P1/P2 only. P3/P4 are non-blocking.

Deferred or out of scope:
Schema and Rust types, APIs/operations, persistence/registry implementation, algorithm catalog beyond the semantics expressly fixed here, expression language, V1 migration/compatibility, delivery packets, rollout, external adopter, parent promotion, posture, and UI.

Validation evidence:
Local Markdown links pass; git diff --check passes; searches find none of the prior contradictory phrases. The exact revised candidate is below.

Do not reduce the task or replace it with an easier alternative. Preserve the requested scope and project conventions.

For this remediation follow-up, report a blocking finding only when it shows an accepted root cause remains unresolved or the remediation introduced a material semantic regression. Do not reopen adjacent product design. Return:
- Findings first with P1-P4 and the exact remaining causal connection.
- Explicitly state whether every accepted P2 is resolved.
- A direct APPROVE or ADJUST verdict.
- If APPROVE, list any P3/P4 advisories compactly without making them required.
- If ADJUST, give only the minimum bounded edits.
- State explicitly when there are no qualifying findings.

REVISED CANDIDATE START

# HCM metric, evaluation, and gate semantic decision

**Status: PROPOSED**

This record fixes the product vocabulary, responsibility boundaries, numeric
meaning, and discriminating scenarios for the Handbook Contract Membrane
metric/evaluation/gate seam. It is a bounded semantic correction after Phase 5
and before Phase-6 consumption. It is not implementation authority until a
human approves it and a later task-level plan separately authorizes delivery.

Current source and the completed [HCM-5.1](slices/HCM-5.1/README.md) and
[HCM-5.2](slices/HCM-5.2/README.md) authorities remain truth for V1. This
record defines the intended replacement semantics; it does not patch V1 or
authorize compatibility or migration work.

## Canonical model and vocabulary

The canonical semantic chain is:

```text
source observation
  -> evidence candidate
  -> Contracts-owned evidence admission
  -> canonical evidence record
  -> applicability and claim evaluation
  -> zero or more named metric results
  -> gate policy
  -> semantic gate decision
  -> separately owned downstream disposition or mutation
```

The following terms are normative. Terms in one row are not synonyms for terms
in another row.

| Term | Fixed meaning |
| --- | --- |
| **Source observation** | A source provider's bounded account of what it observed, including source-native value, unit or scale, subject/snapshot identity, collection state, and provenance. It is not canonical evidence and cannot satisfy a claim by itself. |
| **Evidence candidate** | A dock- or adapter-produced, evidence-shaped submission that binds one or more source observations to an intended evaluation context. It is untrusted input to admission. Do not call it an evidence record or shorten it to “evidence” where that would imply admission. |
| **Canonical evidence record** | The immutable, content-addressed record produced only by Contracts-owned admission after exact identity, provenance, authority, subject, context, unit/scale, normalization, admission-time source validity, and duplicate checks. “Evidence record” means this admitted object. |
| **Applicability result** | The Contracts-owned determination, made before evidence selection, that a claim is `applicable`, `not_applicable`, or `undetermined`, with an exact basis. `not_applicable` is positive semantic proof, not missing data. |
| **Claim evaluation** | The complete result for exactly one expected claim in one evaluation context: its applicability result and, when applicable, an established finding or an indeterminate determination, plus the independent evidence-assessment dimensions and complete accounting. |
| **Finding** | The semantic satisfaction result established by a claim evaluator: `satisfied`, `unsatisfied`, or a value on a claim-declared named grade/measurement scale. There is no universal partial-satisfaction scalar. |
| **Metric definition** | An immutable, named and versioned configured algorithm definition over a declared population of compatible claim evaluations, with explicit input contract, scale, unit, aggregation, parameters, and missing/indeterminate-input policy. Applying it in an evaluation context is the invocation that produces a metric result. |
| **Metric result** | The exact output of one metric definition: `computed` with a value on its named scale, `not_applicable`, or `not_computable`, plus included/excluded inputs and a reproducible calculation trace. There is no unqualified “aggregate score.” |
| **Gate policy** | An immutable semantic decision rule over direct claim-evaluation predicates and named metric-result predicates, including condition precedence and treatment of indeterminacy. |
| **Semantic gate decision** | The non-mutating result `pass`, `fail`, `indeterminate`, or `not_applicable`, with every condition result and decisive reason. It answers the policy's bounded semantic question only. |
| **Downstream disposition** | A separately owned enforcement, workflow, posture, or promotion interpretation of a semantic gate decision, such as allow, block, hold, or no effect. A mutation, if any, remains another authority-controlled operation. |

An **evaluation context** fixes one contract revision, subject snapshot,
evaluation time, evidence-set identity, claim-evaluator versions, metric
definitions, and gate-policy revision. Results that do not share that closure
cannot be combined.

Admission-time source validity is an immutable historical fact about the
candidate-to-record transition. Evaluation-time currentness is recomputed for
the evaluation time under the claim's freshness policy; admission never makes a
durable claim-currentness decision.

## Independent dimensions

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

## Measurement and metric semantics

Every metric definition must state all of the following; omission makes the
definition invalid rather than supplying a default:

1. a stable metric name and algorithm version;
2. the exact eligible claim-evaluation population and compatibility rules;
3. the input and output scale, including bounds, direction, and permitted
   operations;
4. the quantity dimension and canonical unit where quantities are involved;
5. the exact aggregation algorithm and parameters;
6. handling for `not_applicable`, indeterminate, insufficient, stale,
   conflicting, uncertain, missing, or incompatible inputs; and
7. complete included/excluded-input and calculation accounting.

Externally stable numeric values are exact. Finite decimals retain their exact
coefficient and scale; ratios retain exact numerator and positive denominator;
quantities retain exact value and unit; named categorical or ordinal scales
retain their scale identity and direction. No producer, adapter, metric, or
transport may implicitly coerce an IEEE-754 value, round a boundary, infer a
unit, average an ordinal scale, or collapse a quantity to a unitless number.
Exact conversions must be named, versioned, dimension-compatible, and visible
in the trace.

Public signed integer coefficients are bounded to
`-9,007,199,254,740,991..=9,007,199,254,740,991`. Counts, normalized weights,
nonnegative metric numerators/denominators, and their published sums are bounded
to `0..=9,007,199,254,740,991`; a semantic domain that requires a positive
value excludes zero. This is a deliberate cross-runtime interoperability
domain: every public integer component is exactly representable by common
binary64-backed consumers. An out-of-bound public input or result refuses
instead of clamping, rounding, wrapping, or changing representation silently.
Threshold comparison is exact and declares whether equality passes.

A weight exists only as a parameter inside a particular named metric
definition. It is not intrinsic to a claim and never means obligation,
criticality, severity, confidence, producer trust, or gate effect. Relative
rational or finite-decimal weights are normalized exactly to the smallest
positive integer vector that preserves their ratios; zero, negative,
normalization, and total-bound failure reject the metric definition.

The only weighted metric semantics defined and authorized by this record are
the named **Weighted Binary Pass Rate** semantics below. No other weighted
computation is implied or authorized; any future weighted metric requires a
separate semantic decision. Its strictly positive weights are exact rationals
or finite decimals. Its schema and algorithm identity/version representation
remain for later planning:

```text
population  = the definition's exact claim population after proven N/A removal
requirement = every remaining member has an established binary finding
numerator   = sum(metric-local weight for satisfied population members)
denominator = sum(metric-local weight for all population members)
value       = numerator / denominator, exactly
```

It rejects graded or incompatible findings. A missing, indeterminate, stale,
conflicting, or incomplete required input makes the result `not_computable`
under its fixed missing-data policy; a legitimately empty all-not-applicable
population makes it `not_applicable`. Neither case yields `1/1` or 100%.

## Responsibility map

| Layer | Owns | Must not decide |
| --- | --- | --- |
| Validators and source providers | Bounded source-specific execution, source observation, source snapshot identity, native values/units/scales, and collection diagnostics. | Applicability, claim findings, canonical admission, metric weights/values, gate policy, or downstream action. |
| Docks and adapters | Protocol execution/framing, exact translation, preservation of source semantics and lexemes, candidate construction, and import diagnostics. | Canonical evidence, invented units or scale direction, silent normalization, claim meaning, applicability, metrics, or gates. |
| Contracts evidence admission | Exact candidate/source/run/subject binding; authority and provenance validation; canonical normalization; unit/scale identity and permitted exact conversion; idempotent duplicate identity; immutable canonical evidence. | Claim satisfaction, criticality, cross-claim aggregation, gate outcome, or workflow action. |
| Claim evaluation | Applicability; complete evidence selection/accounting; sufficiency, currentness, consistency, uncertainty treatment; semantic interpretation; and one finding or indeterminate determination per expected claim. | Cross-claim metric aggregation, global thresholds, gate precedence, enforcement, posture, promotion, or mutation. |
| Contracts-owned named metric-definition authority | Canonical establishment, exact re-admission validation, and exact resolution of immutable named metric definitions; validation of exact name/version, the declared eligible-population rule and compatibility contract, input/output scales and units, algorithm identity, parameters, strictly positive exact weights and numeric bounds, aggregation semantics, and missing/indeterminate-input policy. | Evidence selection, finding interpretation or establishment, the actual evaluation-run population, metric computation, threshold/comparator application, gate decision, or downstream mutation. |
| Metric computation | Pure exact application of one exactly resolved named metric definition to compatible claim evaluations in one closed evaluation context, including derivation and accounting of the actual included/excluded population and calculation result. | Raw-source/evidence selection, applicability or finding changes, definition changes, gate threshold/comparator application, gate decision, or downstream disposition. |
| Gate policy | Direct non-compensatory claim-evaluation predicates, named metric-result predicates, precedence, indeterminacy treatment, and the semantic gate decision. | Raw evidence interpretation, unit/scale conversion, changing metric semantics, canonical admission, or any workflow/posture/promotion mutation. |
| Downstream consumers | Their own versioned mapping from the exact semantic decision to enforcement, posture, workflow, or promotion disposition; any separately authorized mutation with fresh authority/currentness checks. | Rewriting the gate decision, treating a metric as the decision, bypassing decisive provenance, or transferring mutation authority into Contracts, a dock, SDK, or transport. |

## Gate and explanation invariants

- A direct claim-evaluation predicate is a non-compensatory predicate over
  exactly one claim evaluation, including its applicability, determination,
  finding, sufficiency, currentness, consistency, or uncertainty dimensions. A
  named metric-result predicate is the separate policy construct over one exact
  metric result. A direct predicate is never encoded only as a large weight.
- Metric results do not compensate for one another unless a future named gate
  policy explicitly declares a compatible composite rule. There is no ambient
  averaging across metrics.
- A gate decision is pure derived semantic truth. It persists or mutates no
  lifecycle, work unit, posture, parent, promotion, release, or deployment
  state and grants no authority to do so.
- The four semantic decisions have fixed meanings: `pass` means the policy
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
- The explanation is canonical decision data, not reconstructed prose. It
  preserves expected claim inventory; applicability basis; every included,
  excluded, disqualified, stale, conflicting, and duplicate evidence binding;
  finding and assessment dimensions; each metric definition, population,
  value/state, conversion, and calculation; every gate condition and
  precedence step; and the decisive reason/accounting chain.

## Discriminating scenario corpus

Each scenario assumes exact identity/provenance and the stated immutable policy
unless the row identifies their failure.

| Scenario and discriminating inputs | Responsible layers | Expected semantic outcome | Invariant proved |
| --- | --- | --- | --- |
| **Hard failure with high aggregate.** One required security claim is established unsatisfied; Weighted Binary Pass Rate is `99/100`. The gate requires that claim satisfied and separately requires the metric at least `95/100`; direct failure is decisive. | Claim evaluator; metric; gate policy. | Metric remains `99/100`; the direct predicate fails; gate decision is `fail`. | A compensatory aggregate cannot override a direct predicate. |
| **Missing versus not applicable.** Claim A is applicable but has no admitted record; Claim B has a proved false applicability rule. The gate requires each applicable required claim determinate and treats an indeterminate direct predicate or dependent `not_computable` metric as `indeterminate` when no fail is established. | Applicability and claim evaluators; metric; gate. | A has insufficient coverage and no finding; its determination is indeterminate. B is `not_applicable`. The dependent metric is `not_computable`; gate is `indeterminate`, not pass. | Absence, omission, and N/A are distinct. |
| **Stale evidence.** The only otherwise satisfying record exceeds its freshness policy at the explicit evaluation time. The gate requires the claim satisfied and treats an indeterminate direct predicate or dependent `not_computable` metric as `indeterminate`. | Claim evaluator; metric; gate. | Currentness is stale; no finding is established; claim determination is indeterminate; dependent metric is `not_computable`; gate is `indeterminate`. | Historical satisfaction is not current satisfaction. |
| **Conflicting evidence.** Two eligible authorized records support opposite findings, no resolver is named, and the gate treats an indeterminate direct predicate or dependent `not_computable` metric as `indeterminate`. | Claim evaluator; metric; gate. | Consistency is conflicting; no finding is established; claim determination is indeterminate; dependent metric is `not_computable`; gate is `indeterminate`. | Neither adapter nor metric may cherry-pick. |
| **Duplicate evidence.** The same candidate identity and bytes are submitted twice and the same closed evaluation context is re-evaluated. | Admission; claim evaluator; metric; gate. | Admission is idempotent to one canonical evidence identity; the finding, metric result, semantic gate decision, canonical identities, and accounting are unchanged. A same-identity/different-bytes submission refuses. | Duplication cannot increase weight or confidence. |
| **Flaky observation.** Ten distinct eligible observations contain nine satisfied and one unsatisfied result; the claim declares unanimity as its stability resolver and declares no sampling model. | Claim evaluator. | Consistency is conflicting; no finding is established; claim determination is indeterminate; no generic “90% satisfied” value is produced. | Repetition needs a named sampling model before scalar aggregation. |
| **Quantitative unit normalization.** Authorized observations report `1 s` and `1000 ms` for the same latency quantity, and admission permits one named exact conversion. | Admission; claim evaluator; metric. | Both normalize to the same exact canonical quantity and produce the same finding and metric value with source units retained in provenance. | Units are semantic and exact, not labels. |
| **Graded external scale.** An imported grade is `1` on a versioned ordinal scale where lower is better; a named claim evaluator maps it to a declared grade. | Adapter; admission; claim evaluator; compatible metric. | Direction is honored from the exact scale definition; a binary metric rejects the grade; only a metric naming that scale/mapping may consume it. | External numbers do not imply direction, binary truth, or weights. |
| **Decimal weights and exact bounds.** A Weighted Binary Pass Rate definition assigns exact relative weights `0.25`, `1.50`, `2.25` to findings satisfied, unsatisfied, satisfied. A single-condition gate requires that metric at least `5/8`, with equality included. | Metric-definition authority; metric computation; gate policy. | Definition weights normalize to `1, 6, 9`; calculation trace records numerator `10`, denominator `16`, and exact result `5/8`; the gate passes on equality. Any coefficient, normalized weight, numerator, denominator, or published sum above `9,007,199,254,740,991` refuses before use. | Weight meaning, computation, threshold, and bounds remain exact and separately owned. |
| **All not applicable.** Every claim in the gate's governed population is proved `not_applicable`; the gate policy declares that positive proof for the complete population makes its question not applicable, and the downstream policy maps that decision to `no_effect`. | Applicability evaluator; metric; gate; downstream consumer. | Metric and gate are both `not_applicable`, never `pass`; downstream disposition is `no_effect`. | Empty populations do not pass vacuously. |
| **Degradation monotonicity.** A prior `pass` depends on one current satisfying record. At a later evaluation time the same subject snapshot, applicability, policy, and other eligible information are unchanged, but that record is now stale; the gate treats the resulting indeterminate direct predicate as `indeterminate`. | Claim evaluator; metric; gate. | No finding is established; the dependent metric is `not_computable`; the re-evaluated gate is `indeterminate`, not `pass`, and changed accounting identifies the stale record. | Loss of the sole eligible basis for pass cannot preserve or improve the gate decision. |
| **Mixed population closure.** Candidate claim evaluations differ in subject snapshot, contract revision, evaluator version, or evaluation time. The gate requires the dependent metric computable and treats `not_computable` as `indeterminate`. | Metric computation; gate. | The metric is `not_computable` with every incompatible input accounted; the gate is `indeterminate`. | Metrics cannot span mixed semantics or ambient time. |
| **Multiple named metrics.** A gate requires both a functional pass-rate threshold and a security-severity threshold. The first predicate passes and the second fails; failure is decisive. | Two metrics; gate policy. | Results remain separate; the security metric predicate is decisive and the gate is `fail`. No hidden cross-metric average exists. | One metric cannot silently compensate for another. |
| **Uncertain boundary.** A latency interval straddles the claim budget, no alternate evaluator is named, and the gate requires that claim satisfied while treating an indeterminate direct predicate as `indeterminate`. | Claim evaluator; dependent metric; gate. | Uncertainty is preserved; no finding is established; claim determination is indeterminate; dependent metric is `not_computable`; gate is `indeterminate`. | Midpoints or favorable endpoints cannot be selected silently. |
| **Unauthorized source.** An otherwise well-formed candidate binds a source actor that lacks current authority for the stated subject and observation class. | Contracts evidence admission. | Admission refuses before claim evaluation and produces no canonical evidence record, finding, metric result, or gate decision. | Structural validity and adapter output cannot substitute for source authority. |

## V1 reconciliation

These names describe current implementation truth but are legacy at the new
semantic seam:

| V1 name | Reconciliation |
| --- | --- |
| `ContractClaimV1` (sometimes described conceptually as `ClaimDefinitionV1`) | Legacy claim shape. Its normative claim identity/evidence requirements remain relevant, but claim-attached gate effect and score weight do not survive as intrinsic claim properties. |
| `EvidenceCandidateV1` | Remains an evidence candidate only; it is never admitted evidence. |
| `EvidenceRecordV1` | Aligns with canonical evidence record, subject to the expanded observation/unit/scale semantics selected by later design. |
| `ApplicabilityOutcomeV1::Blocked` | Legacy conflation. The canonical applicability term is `undetermined`; fail-closed gate/downstream behavior is decided separately. |
| `ClaimVerdictV1` / `ClaimVerdictDecisionV1` | Legacy compound result. Canonical claim evaluation separates applicability, established finding versus indeterminate determination, and assessment dimensions. |
| `GateEffectV1` | Legacy claim-attached policy. Non-compensatory claim predicates belong to a gate policy; criticality/severity remains separate. |
| `ScoreWeightV1` | Legacy claim-attached parameter. Weights belong only to a particular named metric definition. |
| `ScoreResultV1` | Legacy unnamed single-ratio result. It is not the universal aggregate. The replacement Weighted Binary Pass Rate has the semantics fixed above; this record does not claim byte or behavioral compatibility with the V1 deficit-counting seam. |
| `GatePolicyV1`, `GateResultV1`, and `GateDecisionV1` | Legacy one-algorithm/two-decision seam. The canonical gate policy consumes direct predicates plus named metric results and emits the four-state semantic gate decision without mutation. |

This reconciliation permits the intended replacement to supersede V1. It does
not select a V1 patch, compatibility compiler, dual run, migration mechanism,
or preserved wire/API shape.

## Settled versus deferred

This record settles the vocabulary and relationships, independent dimensions,
exact measurement rules and bounds, weight ownership, responsibility map, gate
purity and precedence, complete explanation/accounting requirement, and every
scenario outcome above. Later work may not reopen those decisions merely by
choosing a representation.

Deliberately deferred to human-approved task-level planning are schema and Rust
type shapes, API and operation shape, concrete algorithm catalog beyond the
named Weighted Binary Pass Rate semantics, expression language, persistence
layout, registry/versioning mechanics, V1 migration or compatibility, delivery
packets, acceptance plan, and rollout. Also out of scope are external adopter
selection, parent-promotion policy, posture algorithms, UI, a metric
marketplace, and a universal confidence score.

Human approval of this proposed record would make it semantic input to later
planning only. It would not by itself authorize implementation, change a phase
or slice status, or establish any delivery outcome.



REVISED CANDIDATE END
~~~

### Response

~~~text
Findings
P1 Critical

None.

P2 Required

None. No qualifying blocking finding remains. The remediation resolves all six accepted P2 root causes, and I found no material semantic regression introduced by the changes. The revised record remains within the bounded semantic scope and continues to defer schema, API, registry mechanics, migration, delivery, posture, promotion, and UI decisions.


P3 Optional — Align one remaining population phrase with the clarified authority split

Minimum phrase: “the exact eligible claim-evaluation population”

This is not a remaining authority conflation. The responsibility map now expressly assigns definition authority only the declared eligible-population rule and compatibility contract, prohibits it from determining the actual evaluation-run population, and assigns actual population derivation to metric computation.


For maximal lexical consistency, line 135 could eventually be changed from:

the exact eligible claim-evaluation population and compatibility rules;

to:

the exact declared eligible-population rule and compatibility contract;

The current text is already semantically disambiguated by the responsibility rows, so this is not required before human review.


P4 Nit — Repair one list’s grammatical parallelism

Minimum phrase: “zero, negative, normalization, and total-bound failure”

A cleaner rendering would be:

zero or negative weights, normalization failure, or total-bound failure reject the metric definition.

This has no semantic consequence.


Accepted-P2 resolution
Accepted P2	Result	Basis
1. Metric-definition authority missing; threshold assigned to the wrong layer	Resolved	The new Contracts-owned authority row owns canonical establishment, exact re-admission validation, exact resolution, and definition validation while expressly excluding actual population derivation, computation, thresholds, gates, and mutation. Metric computation is separately pure. The decimal scenario now places weights in the definition and threshold/comparator in gate policy.



2. Gate-condition taxonomy and gate not_applicable incomplete	Resolved	Direct claim-evaluation predicates and named metric-result predicates are separately defined. All four decisions now have fixed meanings, and missing input or inability to compute cannot produce not_applicable.



3. Applicability, finding, and indeterminate determination conflated	Resolved	Complete accounting now requires one applicability result per expected claim and, only for an applicable claim, either an established finding or an indeterminate determination. undetermined applicability establishes no finding. The affected scenarios use the corrected terminology.



4. Scenarios non-decisive and representation terms leaked	Resolved	Gate-reaching scenarios now state the relevant predicates, indeterminacy treatment, precedence, and exact outcomes. Unanimity replaces undefined consensus; the weight scenario supplies exact findings, ratio, threshold, and comparator; degradation is one fixed case; “gate bytes” and “receipt” are gone; responsible layers are complete.



5. Weighted Binary Pass Rate exclusivity contradicted the deferred catalog	Resolved	The record now says only that Weighted Binary Pass Rate is the weighted semantics authorized by this record, while expressly allowing a future weighted metric through a separate semantic decision. Its exact algorithm and failure states remain bounded and coherent.



6. Degradation monotonicity overbroad and constrained downstream disposition	Resolved	Degradation is now narrowly defined over fixed subject, applicability, policy, and other eligible information; the guarantee is gate-level only; changed accounting must remain visible; downstream ordering is returned to consumer policy. The scenario fixes one stale-evidence transition and one exact indeterminate outcome.


Verdict: APPROVE

Every accepted P2 is resolved. There are no P1 or P2 findings, no accepted root cause remains, and the remediation introduces no material semantic regression. The two advisories above are non-blocking wording improvements only; no further edit is required before human review.
~~~

## Local reconciliation

The initial ADJUST verdict identified six P2 findings. Five were internal to
the candidate. For the sixth, the reviewer correctly declined to guess the
metric-definition owner without repository facts. Current HCM authority and live
Contracts policy-registry source establish one existing Contracts semantic owner
for exact policy identity, resolution, and evaluation. The clarification round
therefore selected a distinct Contracts-owned metric-definition-authority
responsibility inside that owner, separate from pure metric computation; it did
not select a new crate, API, registry implementation, or mutable ingest
operation.

All six P2 findings were remediated. The follow-up verdict was APPROVE, with no
P1 or P2 finding and no material remediation regression. Its two non-blocking
wording advisories were applied after review: the metric-definition population
phrase now matches the authority split, and the weight-failure list is
grammatically parallel. Neither changes the reviewed semantics.
