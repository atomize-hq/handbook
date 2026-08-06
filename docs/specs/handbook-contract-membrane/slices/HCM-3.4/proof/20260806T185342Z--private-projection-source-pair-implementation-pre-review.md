# HCM-3.4 private Projection source-pair implementation: pre-review proof

Status: candidate implementation subject; independent discovery review pending.

## Authority and scope

This proof records the nonce-bound HCM-3.4 implementation selected by
`20260806T183318Z--private-projection-source-pair-implementation-selector.md`.
The reviewed planning handoff
`20260806T180300Z--HCM-3-4--orchestration--private-projection-source-pair-planning-completed`
is the source authority. The implemented subject is limited to the private
Projection engine, its existing private fixture, one new test-only
`snapshot_current`/`snapshot_delta` fixture, and Projection unit tests.

No public API, CLI, SDK, pipeline, handoff or packet consumer, runtime
adapter, schema, dependency, HCM-3.5/HCM-3.6, or Phase-3 surface is selected.
The frozen one-source path remains in the same configuration and execution
model.

## Implemented private seam

- Normalizes exactly one legacy `captured_revisions` tuple or a sorted,
  nonempty `captured_family_revisions` closure per source.
- Binds `snapshot_current` and `snapshot_delta` by exact source pairs, a
  `to_snapshot` dependency, and immutable current state provenance before
  semantic evaluation, currentness, or payload access.
- Requires five captured/live currentness families, preserving all slots in a
  family check; incompatible, omitted, duplicate, malformed, stale, and
  substituted pair inputs fail closed with typed errors.
- Keeps adapter and source provenance as exact pairs, includes source-pair
  requirements in the validated-definition fingerprint, and preserves
  `authority_effect: none`.

## Test-only proof matrix

The focused Projection test module covers:

- source-pair fixture normalization plus raw legacy/multi-family XOR refusal;
- exact current/delta declaration and five-family configuration admission;
- positive captured/live five-family currentness, including two `work` slots;
- missing source, current-state substitution, malformed delta dependency,
  omitted family, adapter substitution, and stale live family failures;
- redacted omission output with no source payload read; and
- existing one-source cardinality, omission, currentness, fingerprint, and
  lossiness behavior.

The new JSON source-pair fixture is test-only and contains no runtime
consumer.

## Executed local checks

| Check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | passed |
| `cargo test -p handbook-engine --lib projection::tests -- --nocapture` | passed: 17 tests |
| `git diff --check` | passed |

An initial full focused run exposed an accidental test setup mix between the
frozen one-source omission test and the new pair fixture. The test-only setup
was restored to its existing one-source request; the repeated focused wall is
green.

## Change-intelligence status

GitNexus MCP and CLI/index are unavailable in this checkout, so required
upstream impact and change-detection results are recorded as unavailable, not
GREEN. A manual read-only symbol/caller inspection identified the selected
Projection definition, source normalization/binding, currentness, semantic,
and execution seams. The pre-authorized task contract and selector are the
authority to edit them; the historical `execute_projection` blast radius is
HIGH, so the implementation is confined to the minimum private seam and
covered by the focused wall above. GitNexus availability and compare-to-main
status remain explicit final-review and closeout evidence gaps rather than
substitutes for a green result.
