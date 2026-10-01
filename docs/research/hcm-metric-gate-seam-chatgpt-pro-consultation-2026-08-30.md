# HCM metric and gate seam — ChatGPT Pro consultation

- Date: 2026-08-30
- Model: GPT-5.6 Sol, Pro reasoning
- Conversation: https://chatgpt.com/c/6a947758-8610-83e9-a48a-5f1b60e5fd29
- Purpose: independent advisory input for defining the contract-membrane metric and gate seam before broad dock and adapter adoption

> Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

## Prompt

We are already working on this project task.

Task/spec:
Define the correct product and domain seam for a contract-membrane system that consumes heterogeneous validator, dock, and adapter evidence and produces explainable metrics and gate decisions before broad adoption.

Current approach (local repository inference; verify it rather than treating it as authority):

- Evidence docks/adapters emit bounded, deterministic evidence and should not own product gate policy.
- Claims have applicability and categorical outcomes.
- A current V1 gate computes one global exact weighted binary pass ratio: positive JSON-safe integer weights, an exact rational threshold, omitted/not-applicable claims excluded, and hard/required blockers taking precedence.
- The implementation supports only one gate algorithm family and effectively treats its weighted score as the aggregate seam.
- Public semantic values reject floating point numbers, favoring canonical deterministic JSON.
- There is not yet a broad real-world scoring contract, so compatibility cost is still relatively low.

Evidence, blockers, constraints, and validation:

- Future evidence families will be heterogeneous: boolean schema conformance, coverage ratios, performance values with units and budgets, security severities and zero-tolerance findings, accessibility checks, compatibility classifications, stale or incomplete observations, flaky/uncertain measurements, and externally supplied scales.
- A single weighted binary ratio risks averaging incomparable dimensions, allowing compensation where none is intended, collapsing uncertainty into satisfaction, and coupling import adapters to an internal scoring representation.
- Integer weights preserve exact canonical arithmetic, but imports with decimal or externally defined weights require normalization and the aggregate result can exceed the JSON-safe integer transport bound unless owner-level bounds are enforced.
- The likely decomposition under consideration is:
  Evidence -> Claim Evaluation -> Named Metric -> Gate Policy -> Gate Decision.
- Candidate separations include: normative obligation/criticality; measurement and unit; satisfaction; epistemic confidence/sufficiency/currentness; aggregation; and the final decision rule.
- The current weighted algorithm might survive as one explicitly named metric such as WeightedBinaryPassRateV1 rather than the universal abstraction.

Please perform an independent architecture and domain-model review. Do not reduce the task or replace it with an easier alternative.

Is the current approach best? Give a direct keep / adjust / pivot recommendation and explain why. Then propose:

1. a concrete domain model and ubiquitous vocabulary;
2. the semantic boundary between evidence producers, claim evaluators, metric algorithms, and gate policy;
3. invariants that prevent incoherent aggregation or confidence inflation;
4. how heterogeneous measurements, units, missing/not-applicable/stale/conflicting evidence, uncertainty, partial credit, non-compensatory blockers, and multiple metrics should be represented;
5. a versioning and migration strategy that preserves canonical deterministic behavior;
6. what should happen to the existing integer-weighted binary ratio;
7. authoring/import guidance for external decimal weights and score scales;
8. scenario-based stress tests that would falsify the model;
9. a bounded near-term planning sequence suitable before broad adapter/dock adoption.

Call out unresolved product decisions explicitly. Distinguish authoritative model recommendations from optional convenience features. Favor explainability, interoperability, and stable contracts over mathematical cleverness.

## Response

### Recommendation: ADJUST

The current V1 algorithm is defensible inside its narrow domain. The current product abstraction is not.

Keep:

- bounded deterministic producer output;
- explicit claim applicability;
- categorical claim findings;
- exact integer/rational arithmetic;
- exclusion of genuinely not-applicable claims;
- non-compensatory blocker precedence;
- canonical, replayable decisions.

Adjust the product seam now:

> The membrane must produce a set of named metric results and a policy decision—not “the aggregate score.”

This is a localized pivot of the aggregation seam, not a rewrite of the architecture. The proposed decomposition is substantially correct, but it needs an explicit evidence-admission boundary:

```text
Source Observation
    ↓
Evidence Submission + Collection Receipt
    ↓
Evidence Admission
    ↓
Claim Evaluation
    ↓
Named Metric Results
    ↓
Gate Policy
    ↓
Gate Decision
```

The existing weighted binary ratio should survive as `WeightedBinaryPassRateV1`, one deliberately limited metric algorithm.

The available project records support the authority direction: Contract, Evidence, Verdict, and Gate are Handbook-owned; raw logs, tool output, and execution receipts are not automatically Evidence, and Collider must not infer a Verdict or Gate from presentation state. They also reserve Dock as a contract-membrane semantic term rather than an unqualified UI-placement term. The broader canonical JSON contract confirms the repository posture of typed integers, deterministic encoding, and rejection of floating-point and decimal JSON numbers unless a distinct canonical decimal type is introduced.

The adviser did not locate a retrievable source containing the complete current gate formula and schema together. Therefore, its algorithm-specific assessment treats the behavior in the prompt as current implementation truth, while its architecture recommendation is independent of that assumption.

### 1. Concrete domain model and ubiquitous vocabulary

The Contract Membrane is an explainable semantic adjudication layer. It:

- admits and normalizes observations as Evidence;
- evaluates applicable Claims;
- derives explicitly named Metrics;
- applies a named Gate Policy;
- emits a replayable Gate Decision.

It is not a universal scoring engine.

| Term | Meaning |
|---|---|
| Contract Revision | An immutable version of the normative contract, including Claims and their evaluator bindings. |
| Claim | One independently evaluable normative assertion about a subject. |
| Obligation | The claim’s normative strength, such as required, recommended, or informational. |
| Criticality | A domain impact classification. It is not a weight, confidence value, or automatic gate effect. |
| Validator | A producer that examines a target and emits observations. |
| Adapter | An implementation that translates an external format into an Evidence Submission. |
| Evidence Dock | The stable domain port through which a producer submits evidence-shaped records. |
| Evidence Submission | A producer-authored candidate observation. It is not yet authoritative Evidence. |
| Collection Receipt | A record of attempted collection, including success, failure, incompleteness, or unsupported behavior. It cannot satisfy a Claim merely by existing. |
| Evidence Admission | The membrane-owned validation, identity binding, normalization, provenance, unit, scale, and trust step. |
| Evidence Record | An admitted, canonical observation that a Claim Evaluator may consume. |
| Applicability Evaluation | The membrane-owned determination that a Claim is applicable, not applicable, or undetermined. |
| Claim Evaluation | The result of interpreting admitted Evidence under one Claim and evaluator version. |
| Finding | The satisfaction result when the Claim can be determined: satisfied, unsatisfied, or explicitly graded. |
| Evidence Assessment | Orthogonal sufficiency, currentness, consistency, and uncertainty information. |
| Metric Definition | A named, versioned algorithm invocation over a declared compatible set of Claim Evaluations. |
| Metric Result | The exact output of one Metric Definition, including its input accounting and explanation. |
| Gate Policy | A named, immutable decision rule over direct Claim conditions and Metric Results. |
| Gate Profile | A use-specific Gate Policy, such as local feedback, pull-request promotion, release, or broad adoption. |
| Gate Decision | The final semantic outcome and enforcement disposition, with all decisive inputs and reasons. |
| Explanation Trace | The canonical chain from Evidence Records through Claim Evaluations and Metrics to the decision. |

Important vocabulary rules:

- A weight is a parameter of a particular Metric Definition. It is not Claim criticality, Claim obligation, evidence quality, confidence, severity, or producer trust.
- A score is not a generic domain object. It is a value on a named scale emitted by a named Metric.
- An omitted Claim is not synonymous with a not-applicable Claim. Omission is an incomplete or invalid evaluation unless the Contract Revision proves that the Claim is outside the evaluated claim inventory.

Recommended conceptual types (not final wire schemas):

```rust
struct ContractRevisionV2 {
    contract_id: ContractId,
    revision: u64,
    claims: Vec<ClaimDefinitionV1>,
    metric_definitions: Vec<MetricDefinitionV1>,
    gate_policies: Vec<GatePolicyV1>,
    canonical_digest: Sha256,
}

struct ClaimDefinitionV1 {
    claim_id: ClaimId,
    obligation: ObligationV1,
    criticality: CriticalityRefV1,
    applicability_evaluator: EvaluatorBindingV1,
    claim_evaluator: EvaluatorBindingV1,
    accepted_evidence_kinds: Vec<EvidenceKindRefV1>,
    output_contract: ClaimOutputContractV1,
    freshness_rule: FreshnessRuleV1,
}
```

`CriticalityRefV1` should identify a named taxonomy and version. Do not bake an assumed universal ordering into the field.

```rust
struct EvidenceSubmissionV1 {
    submission_id: SubmissionId,
    evidence_kind: EvidenceKindRefV1,
    subject_ref: SubjectRefV1,
    observation: SourceObservationV1,
    producer: ProducerIdentityV1,
    producer_version: String,
    source_snapshot_ref: SourceSnapshotRefV1,
    captured_at: TimestampV1,
    provenance: ProvenanceV1,
    intended_claim_refs: Vec<ClaimId>,
}

struct EvidenceRecordV1 {
    evidence_id: EvidenceId,
    admitted_from: SubmissionId,
    subject_ref: SubjectRefV1,
    evidence_kind: EvidenceKindRefV1,
    original_observation: SourceObservationV1,
    normalized_observation: CanonicalObservationV1,
    admission_profile: AdmissionProfileRefV1,
    admission_version: u64,
    normalization_receipt: NormalizationReceiptV1,
    provenance: ProvenanceV1,
    canonical_digest: Sha256,
}

struct EvidenceCollectionReceiptV1 {
    collection_id: CollectionId,
    producer: ProducerIdentityV1,
    subject_ref: SubjectRefV1,
    status: CollectionStatusV1,
    diagnostic_codes: Vec<DiagnosticCodeV1>,
    attempted_at: TimestampV1,
}
```

A collection failure may explain why a Claim is indeterminate, but it does not prove the Claim false or true.

Use a sum type so invalid applicability/determination combinations cannot be serialized:

```rust
enum ClaimEvaluationOutcomeV1 {
    NotApplicable {
        reason_code: ReasonCodeV1,
        basis_refs: Vec<ArtifactRefV1>,
    },
    ApplicabilityUndetermined {
        reason_codes: Vec<ReasonCodeV1>,
        basis_refs: Vec<ArtifactRefV1>,
    },
    Applicable {
        evidence_assessment: EvidenceAssessmentV1,
        determination: ClaimDeterminationV1,
    },
}

enum ClaimDeterminationV1 {
    Established {
        finding: ClaimFindingV1,
        observed_values: Vec<CanonicalObservationV1>,
    },
    Indeterminate {
        reason_codes: Vec<IndeterminateReasonV1>,
    },
}

enum ClaimFindingV1 {
    Satisfied,
    Unsatisfied,
    PartiallySatisfied {
        grade: NamedScaleValueV1,
    },
}

struct EvidenceAssessmentV1 {
    sufficiency: SufficiencyV1,
    currentness: CurrentnessV1,
    consistency: ConsistencyV1,
    uncertainty: Option<MeasurementUncertaintyV1>,
}
```

Recommended categorical epistemic values:

- sufficiency: `sufficient`, `insufficient`
- currentness: `current`, `stale`, `unknown`
- consistency: `consistent`, `conflicting`, `resolved_by_named_rule`, `not_assessed`

Do not add a generic scalar confidence field. Where a domain genuinely produces confidence, probability, error bounds, or calibration, represent that as a named measurement with its method and version.

```rust
struct RationalV1 {
    numerator: SafeIntegerV1,
    denominator: PositiveSafeIntegerV1,
}

struct QuantityV1 {
    value: RationalV1,
    unit: UnitRefV1,
}

struct IntervalQuantityV1 {
    lower: QuantityV1,
    upper: QuantityV1,
    bound_kind: IntervalBoundKindV1,
}

struct NamedScaleValueV1 {
    scale_ref: ScaleRefV1,
    value: ScaleMemberV1,
}

struct CanonicalDecimalV1 {
    coefficient: SafeIntegerV1,
    scale: u32,
}
```

Rationals are always reduced and have a positive denominator. Finite external decimals should be parsed as exact coefficient-and-scale values, never as floating point. The canonical value can then be converted exactly to `RationalV1`; the import receipt should preserve the original decimal representation.

```rust
struct MetricDefinitionV1 {
    metric_id: MetricId,
    algorithm: MetricAlgorithmRefV1,
    input_claim_ids: Vec<ClaimId>,
    input_contract: MetricInputContractV1,
    parameters: MetricParametersV1,
    output_scale: ScaleRefV1,
    definition_digest: Sha256,
}

enum MetricResultStateV1 {
    Computed { value: NamedScaleValueV1 },
    NotApplicable { reason_code: ReasonCodeV1 },
    NotComputable { reason_codes: Vec<ReasonCodeV1> },
}

struct MetricResultV1 {
    metric_id: MetricId,
    algorithm: MetricAlgorithmRefV1,
    state: MetricResultStateV1,
    included_inputs: Vec<MetricInputTraceV1>,
    excluded_inputs: Vec<MetricExclusionTraceV1>,
    calculation_trace: CalculationTraceV1,
    canonical_digest: Sha256,
}

struct GateDecisionV2 {
    gate_policy_ref: GatePolicyRefV1,
    evaluation_context_ref: EvaluationContextRefV1,
    semantic_outcome: GateOutcomeV1,
    enforcement_disposition: GateDispositionV1,
    condition_results: Vec<GateConditionResultV1>,
    decisive_reason_codes: Vec<ReasonCodeV1>,
    claim_evaluation_refs: Vec<ArtifactRefV1>,
    metric_result_refs: Vec<ArtifactRefV1>,
    explanation_trace_ref: ArtifactRefV1,
    canonical_digest: Sha256,
}
```

Recommended gate outcomes are `pass`, `fail`, `indeterminate`, and `not_applicable`. Recommended enforcement dispositions are `allow`, `block`, `hold`, and `no_effect`. For a broad-adoption gate, normally map pass to allow, fail to block, indeterminate to hold or block according to frozen policy, and not applicable to no effect. This avoids falsely calling missing evidence a product failure while still failing closed operationally.

### 2. Semantic boundaries

| Layer | Owns | Must not own |
|---|---|---|
| Validator | Source-specific observation, bounded execution, source snapshot identity, collection diagnostics | Claim applicability, Claim satisfaction, weights, thresholds, gate policy |
| Adapter | Exact translation from an external representation, source-scale preservation, schema mapping, import diagnostics | Invented units, inferred criticality, silent rounding, satisfaction, gate behavior |
| Evidence Dock | Stable submission protocol, supported Evidence kinds, envelope validation boundary | External source semantics, Claim evaluation, metric behavior |
| Evidence Admission | Trust/profile checks, subject binding, provenance validation, canonical normalization, unit and scale identity, duplicate identity | Claim satisfaction, aggregation, gate outcome |
| Claim Evaluator | Applicability, evidence selection, semantic interpretation, sufficiency, currentness, conflict handling, finding | Cross-Claim aggregation, global thresholds, release or adoption policy |
| Metric Algorithm | Pure derivation from compatible Claim Evaluations, input accounting, exact calculation, output scale | Applicability changes, evidence selection from raw sources, blocker policy, final decision |
| Gate Policy | Non-compensatory requirements, acceptable indeterminacy, metric thresholds, condition precedence, enforcement disposition | Reinterpreting raw Evidence, changing units, inventing partial credit, changing Metric semantics |
| Collider or other presentation | Rendering, navigation, explanation, authoring assistance | Durable semantic authority, inferred Verdicts or Gates |

A Validator and Claim Evaluator may run in the same process for convenience, but their outputs must remain separate artifact types with separate versioned responsibilities. Likewise, an Adapter may submit records that are admitted automatically under a trusted admission profile, but that does not make the Adapter the semantic authority for Evidence admission.

### 3. Invariants preventing incoherent aggregation or confidence inflation

1. **Complete claim accounting.** For every Claim in the evaluated Contract Revision, exactly one must be true: explicitly applicable and evaluated; explicitly not applicable with a reason; applicability undetermined; or evaluation incomplete/invalid. A Claim must never disappear because an Adapter omitted it.
2. **Not-applicable authority.** Only the Claim applicability evaluator may produce `not_applicable`. An external `status: not_applicable` is an observation until the evaluator accepts its meaning.
3. **Satisfaction and epistemic state remain separate.** `unsatisfied` means sufficient current evidence establishes non-satisfaction. `indeterminate` means the system cannot establish either satisfaction or non-satisfaction. Stale, conflicting, missing, malformed, or uncertain evidence must not become partial satisfaction merely to keep a score computable.
4. **No generic confidence multiplier.** Evidence confidence must not be multiplied into satisfaction, weights, numerator contribution, or criticality. Repeated copies of one observation must not increase confidence. Independent repetitions matter only under a named sampling model with proven lineage and independence requirements.
5. **Type-closed aggregation.** Every Metric Algorithm declares the exact input type it accepts, such as binary established findings, a graded named scale, a time quantity, a population-defined ratio, or a versioned severity taxonomy. Incompatible outputs are rejected, not coerced.
6. **Unit closure.** Quantities may be compared or aggregated only when dimensions are compatible, a named exact conversion exists, and the normalized unit is recorded. Latency and memory cannot enter the same arithmetic merely because both are performance concerns.
7. **Scale closure.** Nominal and ordinal categories are not averaged by default. Arithmetic over an external scale requires a named contract that declares type, bounds, direction, anchors, permissible operations, and versioned normalization.
8. **Non-compensatory conditions are direct conditions.** A required security Claim is not represented only as a high weight. Direct gate policy conditions cannot be overridden by a compensatory Metric.
9. **No vacuous passing.** A Metric with no eligible inputs returns `not_applicable` when the complete input set legitimately does not apply, or `not_computable` for missing, undetermined, incompatible, or incomplete inputs. It does not return 1/1 or 100% by empty-set convention.
10. **Evidence degradation cannot improve the decision.** Absent a genuine applicability change, deleting or staling Evidence, lowering completeness, introducing conflict, losing provenance, or widening uncertainty cannot move fail/indeterminate to pass.
11. **Exact arithmetic and bounded public values.** Public values use exact typed forms and an explicit safe-integer range (normally ±9,007,199,254,740,991). Authoring preflights worst-case numerators, denominators, total weights, and normalized parameters. Overflow and out-of-bound normalization are errors, never reasons to round.
12. **Version closure.** A Metric Result consumes Claim Evaluations bound to one Contract Revision, subject snapshot, evaluation context, evaluator version set, and Evidence-set identity. A Gate Decision consumes the exact Metric Definitions and Gate Policy revision named in its context.
13. **No ambient time.** Freshness uses an explicit canonical `evaluation_as_of`, so the same input set and versions reproduce the same bytes.
14. **Explanation completeness.** Every decision exposes the complete expected Claim inventory, applicability, Evidence assessment, finding/indeterminate reason, each Metric’s included/excluded inputs, every Gate condition, decisive reasons, and precedence. Explainability is data, not reconstructed prose.

### 4. Heterogeneous observations and outcomes

| Concern | Recommended representation |
|---|---|
| Boolean schema conformance | Boolean observation; applicable finding is satisfied or unsatisfied. |
| Coverage | Exact numerator and denominator plus named population semantics; no rounded percentage as truth. |
| Performance | `QuantityV1` or `IntervalQuantityV1`, preserving canonical and source units. |
| Budget | Claim Evaluator comparison against a named budget, with observation and comparison trace retained. |
| Security severity | Named taxonomy/version and category/count observations; zero tolerance becomes a direct condition. |
| Accessibility | Named rule/check IDs, conformance categories, and counts, not arbitrary numeric weights. |
| Compatibility | Named classification taxonomy or set membership, without assumed ordering. |
| Missing observation | No Evidence Record; Collection Receipt may explain the gap; Claim is insufficient. |
| Not applicable | Explicit applicability evaluation with reason and basis. |
| Stale Evidence | Currentness is stale; it cannot establish the Claim under the freshness rule. |
| Conflicting Evidence | Consistency is conflicting; Claim is indeterminate unless a named resolver acts. |
| Incomplete Evidence | Sufficiency is insufficient; the Metric denominator is not silently shrunk. |
| Uncertain numeric measurement | Exact interval, error bound, or method-specific uncertainty object. |
| Flaky result | Exact sample inventory or summary, runs, successes/failures, and sampling method; not generic partial satisfaction. |
| Partial credit | `PartiallySatisfied` with a named scale, legal only when declared by the Claim output contract. |
| External score | Preserve source scale identity/value; normalize only through a named versioned mapping. |
| Multiple metrics | Separate `MetricResultV1` artifacts; no synthetic universal total unless a named algorithm deliberately defines it. |

For an interval-valued measurement, the conservative default is: entire interval satisfies → satisfied; entire interval violates → unsatisfied; interval crosses the boundary → indeterminate. Any other treatment requires a named evaluator.

Do not average ratios unless population semantics permit it. Coverage 1/1 plus 50/100 is 51/101, not an unweighted 3/4.

Conflicting sources default to indeterminate. A Claim may name a deterministic resolver such as `NewestAuthorizedObservationV1`, `HighestTrustProfileThenNewestV1`, or `IndependentQuorumV1`; the resolver belongs to the evaluator version, not the Adapter.

Do not create a universal [0,1] satisfaction value. A partially satisfied Claim identifies its scale, and a consuming Metric names the exact mapping. `WeightedBinaryPassRateV1` rejects graded inputs.

### 5. Versioning and migration

Keep these distinct:

| Version or identity | Purpose |
|---|---|
| Evidence schema version | Wire shape of submission/record |
| Producer identity/version | What created the observation |
| Admission profile/version | How it was validated and normalized |
| Claim evaluator ID/version | Applicability and satisfaction semantics |
| Metric algorithm ID/version | Calculation semantics |
| Metric definition revision/digest | Selected Claims, parameters, weights, scale |
| Gate policy revision/digest | Conditions and precedence |
| Contract revision/digest | Normative Claim inventory and bindings |
| Evaluation context digest | Subject snapshot, Evidence set, as-of, and revisions |

Changing an algorithm’s semantics requires a new algorithm version. Changing only the Claim set, weights, threshold, or parameters requires a new immutable Metric Definition revision.

Canonical determinism rules:

- closed schemas; unknown and duplicate fields rejected;
- exact typed numeric objects;
- rationals reduced, positive denominator;
- unordered sets sorted by stable semantic ID;
- order-sensitive policy conditions preserve authored order;
- no ambient time, network, randomness, or mutable registry lookup during replay;
- registry entries referenced by immutable version or digest;
- output commitment includes all input artifact references.

Migration strategy:

1. **Freeze current behavior.** Name it explicitly (for example `LegacyWeightedBinaryGateV1`) and commit golden cases for threshold boundaries, blocker precedence, omission/N/A, zero eligible Claims, maximum weights, ordering, malformed values, and overflow. Do not change V1 semantics while extracting it.
2. **Introduce a new evaluation envelope.** Add `ContractEvaluationV2`/`GateDecisionV2` with `claim_evaluations[]`, `metric_results[]`, `gate_decision`, and `explanation_trace`; no unqualified top-level score.
3. **Map V1 into V2 without reinterpretation.** Binary pass/fail becomes an established binary finding; N/A is explicit applicability; integer weight is a `WeightedBinaryPassRateV1` parameter; threshold is a rational metric condition; hard/required Claims are direct conditions; the aggregate ratio is one Metric Result. Ambiguous omission fails migration rather than being guessed as N/A.
4. **Dual-run.** Compare legacy evaluation and V2 compatibility policy across outcome, ratio components, and blocker precedence; emit migration-equivalence receipts. Preserve public/persisted legacy bytes where required.
5. **Change the default before broad adoption.** Freeze V1; make V2 the default authoring model; require new Docks/Adapters to target Evidence Submission and V2 contracts; retain V1 replay/read support; stop creating new unqualified weighted-gate configurations.

### 6. Treatment of the existing weighted binary ratio

Keep it as `WeightedBinaryPassRateV1` with a deliberately narrow domain.

Given a fixed input Claim set:

```text
eligible = applicable AND established AND binary-valued AND selected
numerator = sum(weight) for satisfied eligible Claims
denominator = sum(weight) for all eligible Claims
result = numerator / denominator
```

The public rational is reduced; the trace may retain unreduced numerator/denominator.

Required behavior:

- not applicable: excluded with explicit reason;
- unsatisfied: contributes zero numerator and full denominator weight;
- partial satisfaction: rejected as incompatible;
- indeterminate: makes the Metric `not_computable` by default;
- missing Claim Evaluation: invalid or `not_computable`;
- zero denominator: `not_applicable` or `not_computable`, never 1/1;
- weights: positive safe integers;
- total weight: bounded at authoring time;
- threshold comparison: exact rational comparison;
- floating-point percentage: never authoritative.

The Metric does not own hard/required blocker semantics, security zero tolerance, Evidence sufficiency, conflict resolution, freshness, final pass/fail, or enforcement disposition. Those belong upstream in Claim Evaluation or downstream in Gate Policy.

For new authoring, exclude non-compensatory blocker Claims from the ratio unless needed for reporting, and evaluate them directly in Gate Policy. Preserve legacy inclusion for compatibility while still evaluating their blocker effect separately.

Example:

```yaml
gate_policy: BroadAdoptionGateV1
direct_conditions:
  - security.no_critical_findings: established_and_satisfied
  - compatibility.required_platforms: established_and_satisfied
  - required_claims: determinate
metric_conditions:
  - functional_quality:
      algorithm: WeightedBinaryPassRateV1
      threshold: 19/20
  - statement_coverage:
      algorithm: CoverageRatioV1
      threshold: 9/10
decision_mapping:
  direct_condition_failure: fail/block
  direct_condition_indeterminate: indeterminate/hold
  metric_not_computable: indeterminate/hold
  metric_below_threshold: fail/block
  all_conditions_satisfied: pass/allow
```

No Metric compensates for another unless a future explicitly named composite policy says so.

### 7. Authoring and import guidance

External decimal weights must be ingested as strings or exact decimal tokens, never through IEEE-754 JSON numbers.

For each weight:

1. preserve the original source lexeme;
2. parse it into `CanonicalDecimalV1`;
3. convert exactly to a rational;
4. normalize the entire vector to the smallest positive integer vector preserving exact ratios;
5. divide by the greatest common divisor;
6. validate positivity and total-weight bounds;
7. record a normalization receipt;
8. reject rather than round if target bounds cannot be met.

Example:

```text
Source:      0.25, 1.50, 2.25
Rationals:   1/4, 3/2, 9/4
Normalized:  1, 6, 9
```

`0.333333` means exactly 333333/1000000, not an inferred 1/3. Scientific notation is acceptable only when parsed exactly into coefficient and scale.

The Metric Definition owner enforces:

```text
every weight > 0
every public weight within SafeIntegerV1
sum(weights) within SafeIntegerV1
every public numerator and denominator within SafeIntegerV1
normalization denominator/scale within authoring limits
```

Excess is invalid; never clamp, approximate, or fall back to floating point.

Every imported scale needs a namespaced, versioned definition with:

- ID/version;
- measurement level: nominal, ordinal, interval, or ratio;
- valid members/bounds;
- direction: higher is better, lower is better, or unordered;
- unit/dimension where applicable;
- anchors/category meanings;
- missing-value meanings;
- normalization mapping when one exists.

Default operations:

| Scale type | Allowed operations by default |
|---|---|
| Nominal | Equality, membership, counts |
| Ordinal | Ordering and thresholds; no averaging |
| Interval | Differences where the scale contract permits |
| Ratio | Exact ratios and weighted arithmetic where units are compatible |

An external priority, severity, confidence, or quality number is not a metric weight merely because it is numeric.

Human-friendly authoring may select Claims by tag/category, but canonical Metric Definitions and Gate Policies freeze resolved Claim IDs, selection-rule version, and resolution digest. Registry changes cannot silently change existing inputs.

### 8. Scenario-based falsification tests

| Scenario | Required result | The model is falsified if… |
|---|---|---|
| 99% score plus one critical security failure | Gate fails/blocks through direct security condition. | High ratio permits adoption. |
| Required Claim has no producer output | Claim insufficient/incomplete; gate indeterminate and holds/blocks. | Claim disappears and gate passes. |
| All selected Claims genuinely N/A | Metric N/A; broad-adoption gate does not vacuously pass. | Result is 100%. |
| One missing Claim, one N/A Claim | States remain distinct in explanation/decision. | Both are silently excluded alike. |
| Passing Evidence becomes stale | Claim becomes indeterminate; decision cannot improve. | Old pass remains authoritative without validity policy. |
| Two authorized sources conflict | Indeterminate unless a named resolver acts. | Adapter/Metric/Gate cherry-picks. |
| Same Evidence submitted twice | Canonical result unchanged; duplicate lineage visible. | Duplication increases confidence or score. |
| Ten flaky runs: nine pass, one fail | Exact sample retained; named sampling evaluator or indeterminate. | Generic “90% satisfied.” |
| Latency in seconds and milliseconds | Exact normalization gives equivalent evaluation. | Results differ or units are ignored. |
| Latency and memory both improve | Separate Claims/Metrics absent explicit composite. | Unnamed performance average. |
| Coverage 1/1 and 50/100 | 51/101 if population-compatible, otherwise reject. | Unweighted average 3/4. |
| Partial accessibility grade enters binary metric | Binary metric rejects input. | Silent coercion to pass/fail/fraction. |
| Decimal weights 0.1/0.2/0.3 | Exact normalization 1/2/3 with receipt. | Floating artifacts affect result. |
| Weight vector exceeds transport bounds | Authoring fails. | Evaluation wraps, rounds, truncates, or emits unsafe JSON. |
| External scale has 1=best | Import rejects or honors declared direction. | Normalized finding is inverted. |
| Ordinal severities are averaged | Reject absent explicit mapping. | Unnamed arithmetic mean appears. |
| Mixed Contract Revisions | Metric computation rejects. | Result spans mixed semantics. |
| Threshold exactly met | Declared inclusive/exclusive comparator controls. | Rounding changes boundary. |
| Evidence/input order permuted | Canonical bytes unchanged where order is non-semantic. | Iteration order changes result. |
| Same inputs evaluated later | Same explicit `evaluation_as_of` reproduces bytes. | Ambient time changes result. |
| Uncertainty interval straddles budget | Indeterminate by conservative default. | Midpoint/favorable endpoint chosen silently. |
| Contradictory Evidence added to pass | Named resolver preserves pass or result degrades to indeterminate. | More contradiction increases confidence. |
| Producer marks Claim N/A | Treated as observation; evaluator decides. | Producer removes Claim from policy. |
| Metric A passes, B fails | Explicit Gate conditions identify B as decisive. | Hidden average lets A compensate for B. |

Two key property families:

```text
Determinism:
  identical canonical inputs -> identical canonical outputs

Evidence-degradation monotonicity:
  removing or degrading evidence cannot improve the gate outcome
  unless an explicit applicability input changed
```

### 9. Bounded near-term planning sequence

1. **Lock the semantic decision record.** Freeze the pipeline, vocabulary, Evidence Submission vs admitted Evidence, applicability states, satisfaction vs epistemic state, exact numeric types, Gate outcome vs disposition, the role of `WeightedBinaryPassRateV1`, and unresolved decisions. Exit: reviewed record, no implementation expansion.
2. **Freeze and characterize the existing gate.** Extract input inventory, formula, constraints, comparator, N/A/omission behavior, blockers, zero denominator, numeric bounds, canonical output, and adversarial fixtures. Exit: replayable behavior that cannot drift.
3. **Define minimum V2 artifacts.** Specify only `EvidenceSubmissionV1`, `EvidenceCollectionReceiptV1`, `EvidenceRecordV1`, `EvaluationContextV1`, `ClaimEvaluationV1`, `MetricDefinitionV1`, `MetricResultV1`, `GatePolicyV1`, `GateDecisionV2`, `RationalV1`, `QuantityV1`, and `ScaleDefinitionV1`. Avoid arbitrary expression languages and plugin marketplaces. Exit: closed schemas, canonical examples, negative examples.
4. **Build one pure reference path.** Evidence admission, one binary Claim Evaluator, `WeightedBinaryPassRateV1`, direct blocker conditions, exact rational threshold, complete Explanation Trace, no ambient I/O. Exit: deterministic golden vectors across Rust and one independent reader/verifier.
5. **Add V1 compatibility compiler.** Map frozen V1 to V2, dual-run, and emit equivalence receipts. Exit: exact equivalence or explicit unmigratable diagnostics for each supported fixture.
6. **Prove three heterogeneous vertical slices.** Schema conformance; performance budget with quantities/intervals/freshness; security with severity taxonomy/conflicts/zero tolerance. Add coverage only if needed to prove population semantics. Exit: falsification tests pass without a universal score.
7. **Define Adapter/Dock conformance kit.** Test deterministic submissions, snapshot identity, source value/scale preservation, exact decimal import, no unauthorized applicability/satisfaction, duplicates, canonical ordering, failed/incomplete collection, unsupported units/scales, and overflow rejection. Exit: Adapter proves conformance without owning Claim/Gate policy.
8. **Broad-adoption readiness review.** Require green vertical slices, closed V1 compatibility, settled public-schema decisions, passing falsification corpus, useful explanations, and no producer-specific leakage into Gate Policy.

### Explicit unresolved product decisions

| Decision | Recommended direction |
|---|---|
| Does Claim criticality automatically imply gate effect? | No. Claim owns obligation/impact; each Gate Policy owns enforcement. |
| Are multiple Gate Profiles first-class? | Yes: at least reporting/local feedback and broad adoption, even if only one ships initially. |
| What does indeterminate do operationally? | Keep semantic outcome indeterminate; broad-adoption policy maps to hold/block. Lock public terms/effects. |
| May producers emit authoritative Evidence directly? | No by default. They emit Submissions; trusted automatic admission remains a separate membrane-owned step. |
| Is general partial credit in first release? | No. Permit only explicitly declared graded Claims required by proven scenarios. |
| How are conflicts resolved? | Default indeterminate; allow only named evaluator-owned resolvers. |
| Can Gate Policy impose stricter freshness? | Prefer evaluator/evaluation profile; Gate can require current evaluation but not reinterpret raw Evidence. |
| What is core criticality taxonomy? | Keep small or namespaced; severity, obligation, and gate effect remain separate axes. |
| How extensible are units/scales? | Closed core plus registered namespaced extensions referenced by immutable version/digest. |
| Arbitrary policy expressions? | Not initially. Use staged closed policy: applicability, direct conditions, sufficiency, metric conditions, decision mapping. |
| Arbitrary-size canonical integers? | No initially. Use bounded safe integers/rationals; add a canonical big integer only from demonstrated need. |
| Can one Metric consume another? | Defer. Initial Metrics consume Claim Evaluations only; composition raises hidden-compensation risk. |
| Does N/A permit adoption? | Not itself. It removes an obligation from applicability; complete Gate Policy decides sufficiency. |
| Final meaning of Verdict? | Reserve as generic Handbook category; use `ClaimEvaluation` and `GateDecision` for concrete artifacts. |

### Optional convenience features, not semantic authority

- authoring CLI that previews Claim selection and decimal normalization;
- “why did this gate fail?” renderer;
- metric comparison dashboard;
- unit/scale lookup autocomplete;
- schema-generated Adapter SDKs;
- what-if simulations using alternate weights/thresholds;
- graphical Evidence lineage;
- prose generation from canonical Explanation Trace;
- catalog of reusable Metric Algorithms.

What-if results must be visibly non-authoritative unless evaluated under a persisted Gate Policy revision.

### Bottom line

> Evidence is observed truth; Claim Evaluation is semantic judgment; Metrics are named summaries; Gate Policy is the decision rule.

Do not let a producer decide applicability. Do not let a Metric decide policy. Do not let uncertainty become partial satisfaction. Do not let a weight mean criticality, trust, and importance simultaneously. Do not let heterogeneous dimensions collapse into an unnamed score.

Adopt the new model before broad Dock and Adapter adoption, while compatibility is still small. Preserve the current ratio exactly, but demote it from universal aggregate to the narrow metric it is: `WeightedBinaryPassRateV1`.
