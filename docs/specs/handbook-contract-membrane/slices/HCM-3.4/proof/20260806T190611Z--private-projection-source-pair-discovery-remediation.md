# HCM-3.4 source-pair discovery remediation

Status: consolidated P2 remediation complete; fresh closure review pending.

## Causal input

This is the one consolidated repair for the discovery review dispatched as
`20260806T185410Z--HCM-3-4--private-projection-source-pair-implementation-discovery`.
It addresses only these P2 findings against the prior manifest:

- `HCM34-SPI-DISC-001`: optional current-state identity;
- `HCM34-SPI-DISC-002`: readable `snapshot_delta` policy/evaluator path; and
- `HCM34-SPI-DISC-003`: missing dependency, selection, currentness-slot, and
  retained-pointer proof cases.

## Consolidated repair

- Rejects a source-pair definition unless `require_state_identity` is true and
  unconditionally verifies the selected current state plus the delta
  `to_snapshot` state dependency.
- Keeps `snapshot_delta` out of the trusted evaluator and disclosure-policy
  source-kind lists. A delta selector is admitted only as the derived member
  of an exact source-pair relation, and a field rule naming it is refused at
  definition validation before any payload read.
- Adds a retained safe objective pointer to the test-only current snapshot and
  proves that it is evaluated separately from a withheld upstream pointer.
- Adds no-state, missing/duplicate `to_snapshot`, duplicate selected source,
  stale work-slot, selector substitution, and slot substitution negatives,
  each before payload access where execution is reached.

No authority, public surface, consumer, schema, dependency, runtime adapter,
or future-slice scope changed. The repaired source-pair definition and
request/result fingerprints retain every selected exact-pair fact.

## Local delta proof

| Check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | passed |
| optional state / delta-payload admission negative | passed |
| source-pair currentness and fail-closed matrix | passed |
| `git diff --check` | passed |

The complete focused Projection wall and broader engine library wall are
re-run before primary commit. This record does not claim closure CLEAN; a
different fresh reviewer must inspect only this repaired delta and the final
subject state.
