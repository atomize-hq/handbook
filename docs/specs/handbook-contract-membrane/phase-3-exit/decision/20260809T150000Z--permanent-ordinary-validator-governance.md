# Permanent ordinary-validator governance

**Status:** accepted permanent operator-governance authority; consumed only
under its exact conditions.

## Historical failure and classification

The preserved historical raw failure is:

```text
20260806T202700Z--HCM-3-5--resolution-aware-adoption-planning: continuation writes or advances before selector CLEAN
```

Its classification is
`accepted_permanent_operator_governance_exception`.

## Duration and relying workflows

This authority is permanent for this repository lineage and every Handbook
planning, implementation, proof, review, validation, audit, closeout,
adoption, publication-preflight, and phase-transition workflow. Reauthorization
is never required again for this exact historical failure.

Every relying selector, proof, and handoff must cite both this decision and the
exact raw failure above. A consumer must preserve the ordinary validator's raw
failed/not-GREEN result; it must not relabel, suppress, patch, or report that
result as a pass.

## Narrow closeout semantics

An otherwise eligible closeout may proceed only when the exact historical
condition above is the sole ordinary-validator failure and every other
applicable validation, proof, review, P1/P2, causal-lineage, scope, protected-
path, and publication-preflight gate passes.

The decision is not a validator patch and does not change historical validator
semantics. The historical HCM-3.5 dispatches, record, ledger history,
admission hashes, and all immutable evidence remain byte-for-byte unchanged.

## Non-waiver

No other validator result, P1/P2, code/test/proof/review failure,
causal-lineage violation, scope expansion, protected-path mutation, or future
defect is waived. Any additional ordinary-validator failure is blocking. A
current strict-lint, build, test, formatting, schema, handoff, proof, review,
or GitNexus failure remains an ordinary blocking result and requires its own
authority-appropriate repair.
