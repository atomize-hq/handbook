# HCM-2.4 P4/P5 generic mutation token authority stop

Status: **historical bounded stop; token recommendation superseded by the
operator-approved selector**

Date: 2026-07-27

## Decision

At this historical baseline, P4 Decision Record and P5 Risk Record retained
their independently selected repository fixtures and available
read/validate/intake/renderer evidence, but neither packet was complete. Their
required positive generic mutation proof was blocked before intake-record
establishment by the then-unchanged HCM-2.3 token contract mismatch.

This record did not authorize a runtime, released definition, schema, token
grammar, public API, dependency, Cargo, version, unsafe-policy, or sibling-slice
change. Its token recommendation was later superseded by
`20260727-p4-p5-coverage-token-derivation-selector.md`. P6 remained closed at
this baseline because P2, P4, and P5 were not GREEN.

## Historical contract mismatch

`artifact_mutation::intake_commit_plan` derived subordinate output tokens from
the suffix of each released coverage ID and appended `-value`. Required
coverage IDs such as `schema_id`, `schema_version`, `record_id`,
`evidence_refs`, and `review_basis` therefore produced underscore-bearing
tokens.

`artifact_lineage_store::valid_intake_value_token` admitted only ASCII
lowercase letters, digits, and hyphens. `validate_output_contract`
consequently rejected the planner-generated tuple with:

```text
ArtifactMutationErrorV1 {
  kind: Store,
  detail: "generic lineage store refused: intake output tuple is not exact or unique"
}
```

Fixture values could not alter the released coverage IDs. Renaming those IDs
would have changed released P1A definitions. Widening the lineage-store grammar
would have changed a CRITICAL surface and was not justified by this evidence.

## Historical proof posture

Each packet then had:

- a default-green target that preserves its available real-path evidence;
- an exact negative regression for the current Store refusal; and
- an ignored positive mutation test that was executable with `--ignored`
  and failed at the exact prerequisite boundary.

The ignored tests were unavailable positive proof, not waived gates. P4 and P5
were incomplete and unpromoted at that baseline.

## Superseded prerequisite recommendation

This record recommended that a future authority packet consider normalizing the
derived coverage-ID suffix inside `intake_commit_plan` from underscores to
hyphens before appending `-value`, while preserving exact collision detection
and every existing negative. That was only a feasibility direction, not
implementation authority.

It required any later selector to:

1. select the exact runtime symbol and all affected tests;
2. rerun exact GitNexus impact at the new baseline;
3. preserve released intake bytes/fingerprints and the existing token grammar;
4. prove no collisions or broadened admission for every released coverage
   vector; and
5. obtain fresh independent implementation review.

The later operator-approved selector selected and implemented only that token
prerequisite. The exact token RED/GREEN proofs are active, different-fresh
supplemental causal closure returned CLEAN, and the reviewed packet is
committed at `00dde01`. P4's separate no-root, generated-command,
inferred-filename, Projection, and persistent-view proof gate remains
outstanding and outside that token selector. This historical record therefore
does not establish P4 completion or P6 authority.

## Evidence

- P4 proof:
  `proof/implementation/P4-decision-record.md`
- P4 stop-review dispatch:
  `../../../handoffs/dispatches/20260727T004037Z--HCM-2-4--p4-decision-record-stop-review.json`
- P4 reviewer: `/root/hcm_2_4_p4_stop_review`, completed with one Required
  runtime-prerequisite finding.
- P5 proof:
  `proof/implementation/P5-risk-record.md`
- P5 stop-review dispatch:
  `../../../handoffs/dispatches/20260727T004038Z--HCM-2-4--p5-risk-record-stop-review.json`
- P5 reviewer: `/root/hcm_2_4_p5_stop_review`, completed CLEAN on the truthful
  partial proof and independently confirmed the same prerequisite.
