# HCM-2.4 P4/P5 generic mutation token authority stop

Status: **accepted bounded stop; runtime correction not authorized**

Date: 2026-07-27

## Decision

P4 Decision Record and P5 Risk Record retain their independently selected
repository fixtures and available read/validate/intake/renderer evidence, but
neither packet is complete. Their required positive generic mutation proof is
blocked before intake-record establishment by an unchanged HCM-2.3 token
contract mismatch.

No runtime, released definition, schema, token grammar, public API, dependency,
Cargo, version, unsafe-policy, or sibling-slice change is authorized by this
record. P6 remains closed because P2, P4, and P5 are not GREEN.

## Live contract mismatch

`artifact_mutation::intake_commit_plan` derives subordinate output tokens from
the suffix of each released coverage ID and appends `-value`. Required coverage
IDs such as `schema_id`, `schema_version`, `record_id`, `evidence_refs`, and
`review_basis` therefore produce underscore-bearing tokens.

`artifact_lineage_store::valid_intake_value_token` admits only ASCII lowercase
letters, digits, and hyphens. `validate_output_contract` consequently rejects
the planner-generated tuple with:

```text
ArtifactMutationErrorV1 {
  kind: Store,
  detail: "generic lineage store refused: intake output tuple is not exact or unique"
}
```

Fixture values cannot alter the released coverage IDs. Renaming those IDs
would change released P1A definitions. Widening the lineage-store grammar would
change a CRITICAL surface and is not justified by this evidence.

## Proof posture

Each packet now has:

- a default-green target that preserves its available real-path evidence;
- an exact negative regression for the current Store refusal; and
- an ignored positive mutation test that remains executable with `--ignored`
  and fails at the exact prerequisite boundary.

The ignored tests are unavailable positive proof, not waived gates. P4 and P5
remain incomplete and unpromoted.

## Smallest separately selectable prerequisite

A future authority packet may consider normalizing the derived coverage-ID
suffix inside `intake_commit_plan` from underscores to hyphens before appending
`-value`, while preserving exact collision detection and every existing
negative. That is only a feasibility direction, not implementation authority.

Before any such edit, the packet must:

1. select the exact runtime symbol and all affected tests;
2. rerun exact GitNexus impact at the new baseline;
3. preserve released intake bytes/fingerprints and the existing token grammar;
4. prove no collisions or broadened admission for every released coverage
   vector; and
5. obtain fresh independent implementation review.

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
