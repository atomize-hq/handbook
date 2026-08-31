# HCM metric, evaluation, and gate semantic decision

**Status: PROPOSED**

This record fixes the product vocabulary, responsibility boundaries, numeric
meaning, and discriminating scenarios for the Handbook Contract Membrane
metric/evaluation/gate seam as a greenfield semantic decision. It is not
implementation authority until a human approves it and later task-level
planning separately authorizes delivery.

Current source and the completed [HCM-5.1](slices/HCM-5.1/README.md) and
[HCM-5.2](slices/HCM-5.2/README.md) authorities are evidence only for semantic
ownership, dependency direction, and trust boundaries. Their current public
shapes and computations neither constrain nor supply this model.

The companion
[posture and assurance-policy semantic decision](posture-assurance-policy-semantic-decision.md)
owns the proposed upstream selection seam. When posture influences assurance
selection, that record's exact snapshot and resolved-requirement provenance are
part of this record's closed evaluation context; neither record is
implementation authority.

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
| **Metric definition** | An immutable, named, versioned, and fully configured algorithm definition over a declared population of compatible claim evaluations, with explicit input contract, scale, unit, aggregation, parameters, and missing/indeterminate-input policy. Applying it in an evaluation context is the invocation that produces a metric result. |
| **Metric result** | The exact output of one metric definition: `computed` with a value on its named scale, `not_applicable`, or `not_computable`, plus included/excluded inputs and a reproducible calculation trace. There is no unqualified “aggregate score.” |
| **Gate policy** | An immutable semantic decision rule over direct claim-evaluation predicates and named metric-result predicates, including condition precedence and treatment of indeterminacy. |
| **Semantic gate decision** | The non-mutating result `pass`, `fail`, `indeterminate`, or `not_applicable`, with every condition result and decisive reason. It answers the policy's bounded semantic question only. |
| **Downstream disposition** | A separately owned enforcement, workflow, posture, or promotion interpretation of a semantic gate decision, such as allow, block, hold, or no effect. A mutation, if any, remains another authority-controlled operation. |

An **evaluation context** fixes one contract revision, subject snapshot,
evaluation time, evidence-set identity, claim-evaluator versions, metric
definition identities and configurations, and gate-policy revision. Results
that do not share that closure cannot be combined.

When posture influenced assurance selection, the evaluation context also fixes
the Resolved Posture Snapshot fingerprint, Assurance Policy Selector identity
and version, and Resolved Assurance Requirements identity. Those bindings must
participate in evaluation fingerprinting, currentness, and canonical
explanation. Metrics and gates consume the resolved requirements; they never
reread or reinterpret mutable posture, reselect requirements, or change metric
meaning. This is the cross-boundary invariant fixed with the
[companion posture/assurance record](posture-assurance-policy-semantic-decision.md).

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

## Responsibility map

| Layer | Owns | Must not decide |
| --- | --- | --- |
| Validators and source providers | Bounded source-specific execution, source observation, source snapshot identity, native values/units/scales, and collection diagnostics. | Applicability, claim findings, canonical admission, metric weights/values, gate policy, or downstream action. |
| Docks and adapters | Protocol execution/framing, exact translation, preservation of source semantics and lexemes, candidate construction, and import diagnostics. | Canonical evidence, invented units or scale direction, silent normalization, claim meaning, applicability, metrics, or gates. |
| Contracts evidence admission | Exact candidate/source/run/subject binding; authority and provenance validation; canonical normalization; unit/scale identity and permitted exact conversion; idempotent duplicate identity; immutable canonical evidence. | Claim satisfaction, criticality, cross-claim aggregation, gate outcome, or workflow action. |
| Claim evaluation | Applicability; complete evidence selection/accounting; sufficiency, currentness, consistency, uncertainty treatment; semantic interpretation; and one finding or indeterminate determination per expected claim. | Cross-claim metric aggregation, global thresholds, gate precedence, enforcement, posture, promotion, or mutation. |
| Contracts-owned named metric-definition authority | Canonical establishment, exact re-admission validation, and exact resolution of immutable named, versioned, configured metric definitions; validation of the eligible-population and compatibility rules, input/output scales and units, algorithm identity/version, parameters and coefficients under their algorithm-declared semantics and bounds, aggregation semantics, and missing/indeterminate-input policy. | Evidence selection, finding interpretation or establishment, the actual evaluation-run population, metric computation, threshold/comparator application, gate decision, or downstream mutation. |
| Metric computation | Pure exact application of one exactly resolved named metric definition to compatible claim evaluations in one closed evaluation context, including derivation and accounting of the actual included/excluded population and calculation result. | Raw-source/evidence selection, applicability or finding changes, definition changes, gate threshold/comparator application, gate decision, or downstream disposition. |
| Gate policy | Direct non-compensatory claim-evaluation predicates, named metric-result predicates, precedence, indeterminacy treatment, and the semantic gate decision. | Raw evidence interpretation, unit/scale conversion, changing metric semantics, canonical admission, or any workflow/posture/promotion mutation. |
| Downstream consumers | Their own versioned mapping from the exact semantic decision to enforcement, posture, workflow, or promotion disposition; any separately authorized mutation with fresh authority/currentness checks. | Rewriting the gate decision, treating a metric as the decision, bypassing decisive provenance, or transferring mutation authority into Contracts, a dock, SDK, or transport. |

## Gate and explanation invariants

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
| **All not applicable.** Every claim in the gate's governed population is proved `not_applicable`; the named metric's definition declares an empty eligible population `not_applicable`; the gate policy declares that positive proof for the complete population makes its question not applicable; and the downstream policy maps that decision to `no_effect`. | Applicability evaluator; metric; gate; downstream consumer. | Metric and gate are both `not_applicable`, never `pass`; downstream disposition is `no_effect`. | Empty populations do not pass vacuously. |
| **Degradation monotonicity.** A prior `pass` depends on one current satisfying record. At a later evaluation time the same subject snapshot, applicability, policy, and other eligible information are unchanged, but that record is now stale; the gate treats the resulting indeterminate direct predicate as `indeterminate`. | Claim evaluator; metric; gate. | No finding is established; the dependent metric is `not_computable`; the re-evaluated gate is `indeterminate`, not `pass`, and changed accounting identifies the stale record. | Loss of the sole eligible basis for pass cannot preserve or improve the gate decision. |
| **Mixed population closure.** Candidate claim evaluations differ in subject snapshot, contract revision, evaluator version, or evaluation time. The gate requires the dependent metric computable and treats `not_computable` as `indeterminate`. | Metric computation; gate. | The metric is `not_computable` with every incompatible input accounted; the gate is `indeterminate`. | Metrics cannot span mixed semantics or ambient time. |
| **Multiple named metrics.** A gate requires both a named functional-metric threshold and a named security-metric threshold. The first predicate passes and the second fails; failure is decisive. | Two metrics; gate policy. | Results remain separate; the security metric predicate is decisive and the gate is `fail`. No hidden cross-metric average exists. | One metric cannot silently compensate for another. |
| **Uncertain boundary.** A latency interval straddles the claim budget, no alternate evaluator is named, and the gate requires that claim satisfied while treating an indeterminate direct predicate as `indeterminate`. | Claim evaluator; dependent metric; gate. | Uncertainty is preserved; no finding is established; claim determination is indeterminate; dependent metric is `not_computable`; gate is `indeterminate`. | Midpoints or favorable endpoints cannot be selected silently. |
| **Unauthorized source.** An otherwise well-formed candidate binds a source actor that lacks current authority for the stated subject and observation class. | Contracts evidence admission. | Admission refuses before claim evaluation and produces no canonical evidence record, finding, metric result, or gate decision. | Structural validity and adapter output cannot substitute for source authority. |

## Settled versus deferred

This record settles the vocabulary and relationships, independent dimensions,
exact measurement rules and bounds, weight ownership, responsibility map, gate
purity and precedence, complete explanation/accounting requirement, and every
scenario outcome above. Later work may not reopen those decisions merely by
choosing a representation.

Deliberately deferred to human-approved task-level planning are all concrete
metric algorithms and catalog entries; schema, type, API, and operation shapes;
expression language; persistence layout; registry and version-resolution
mechanics; any cutover or temporary internal bridge; implementation packets;
acceptance planning; and rollout. Also out of scope are external adopter
selection, parent-promotion policy, posture algorithms, UI, a metric
marketplace, and a universal confidence score.

Human approval of this proposed record would make it semantic input to later
planning only. It would not by itself authorize implementation, change a phase
or slice status, or establish any delivery outcome.
