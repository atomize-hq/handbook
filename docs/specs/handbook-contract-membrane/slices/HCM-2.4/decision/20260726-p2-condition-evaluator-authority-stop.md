# HCM-2.4 P2 condition-evaluator authority stop

Status: accepted after independent clean review; runtime prerequisite remains open

Date: 2026-07-27

## Context

P2 requires Environment Context authoring to write selected
`.handbook/project/environment.yaml` only when
`handbook.condition.project.managed-operational-surface@1.0.0` evaluates
`true`, and its GREEN wall requires both `true` and `false` proof.

The frozen HCM-0.6 decision defines six outcomes and explicitly leaves the
condition record schema, exact input bindings, admitted evidence types,
outcome precedence, freshness thresholds, evaluator implementation, transport,
and migration behavior undecided. It also excludes Environment Context as its
own applicability input and rejects bare silence or unchecked profile flags as
`false`.

Live repository truth matches that freeze:

- `ResolvedProfileDecisions::from_profile` routes every conditional descriptor
  through `artifact_decision`;
- `artifact_decision` emits only `ProjectConditionOutcome::Unresolved`,
  `ProjectConditionDecisionReason::EvidenceContractUnavailable`, and
  `ArtifactApplicability::Indeterminate`; and
- the Environment Inventory author path calls
  `resolve_shipped_profile_decisions` and exposes no independently resolved,
  admitted condition-evidence input.

P2's reviewed selector does not include `profile_decision.rs`, a new condition
evaluator, or an evidence-contract surface. Therefore the positive authoring
contract is not executable within current authority.

## Decision

Pause P2 behavior-changing work. Preserve its condition-true/false,
write-only-when-applicable, conditional-absence, and fail-closed exit
requirements without weakening them.

Specifically, P2 must not:

- coerce `indeterminate` to `true` or `false`;
- treat structured Environment Context input, `applicability_basis`, selected
  profile membership, artifact presence, or profile opt-in as independent
  condition evidence;
- let Environment Context authorize its own applicability;
- write canonical Environment Context YAML while applicability is unresolved;
  or
- edit `profile_decision.rs` or add an evaluator/evidence surface without
  separately approved exact authority and selector coverage.

P2 may resume only when a fingerprinted condition-evidence/evaluator contract
with exact inputs, admitted evidence, precedence, freshness, and proof either:

1. enters HCM-2.4 through a separately reviewed selector amendment; or
2. is completed elsewhere and proven as a dependency.

P3–P5 remain semantically independent and may proceed serially. P6 remains
blocked because its entry requires P2–P5 replacement proof to be GREEN.

Independent review:

- dispatch:
  `20260727T000736Z--HCM-2-4--p2-condition-authority-stop-review`;
- built-in reviewer:
  `/root/hcm_2_4_p2_condition_authority_review`;
- final status: `completed`;
- verdict: `clean`; and
- findings/advisories: none.

## Consequences

- No production, test, template, schema, profile, or released-definition byte
  is changed by this decision.
- The existing P2 path selector remains closed and unchanged except for an
  explicit read/proof-only exclusion for condition evaluation.
- The six-outcome policy remains fail-closed.
- The top-level HCM-2.4 run can complete other independently authorized packets
  but cannot complete P2, begin P6, or close the slice until this authority
  prerequisite is satisfied.

## Rejected alternatives

- Using `applicability_basis` would be self-referential because the candidate
  Environment Context record is excluded from its own applicability inputs.
- Treating profile selection as `true` would collapse selection into
  requiredness and contradict the frozen semantic model.
- Treating missing evidence as `false` would violate the exact six-outcome
  policy.
- Writing while `indeterminate` would violate P2's write-only-when-applicable
  contract and create authority before applicability is proven.
