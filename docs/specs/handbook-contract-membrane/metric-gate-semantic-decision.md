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
