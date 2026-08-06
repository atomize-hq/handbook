# HCM-3.4 private Projection source-pair implementation selector

**Status:** active, nonce-bound implementation selector for one private
generic Projection source-pair outcome. This selector is authorized by the
explicit HCM-3.4 source-pair implementation contract bound to dispatch nonce
`04b65dd6af294e8fbd7439eb1dd174a60cec7915e3514bd0b4cddd91a41cb5f3`.

## Selected outcome and predecessor truth

This selector consumes the completed planning handoff
`20260806T180300Z--HCM-3-4--orchestration--private-projection-source-pair-planning-completed`
and the immutable cardinality true stop
`20260806T064500Z--HCM-3-4--orchestration--snapshot-memory-projection-cardinality-stop`.
It selects exactly one new integrated implementation outcome:

| Integrated outcome | Packet | Authority |
|---|---|---|
| `hcm-3.4-private-projection-source-pair-implementation` | `HCM-3.4-I2-private-projection-source-pair-implementation` | this selector, the completed planning handoff, and `tasks/plan.md` |

The preceding planning outcome, its three remediated P2 findings, and its
CLEAN closure stay immutable evidence. This is not a continuation of that
planning review budget and does not amend the blocked Snapshot Memory parent.
The new implementation outcome begins with one discovery review, permits one
consolidated P1/P2 remediation and one different-fresh closure review, and
permits at most two directly causal supplemental cycles. No review follows a
CLEAN verdict.

## Exact owner and path envelope

The only production owner is the private `handbook-engine` Projection module:

- `crates/engine/src/projection.rs` — `AuthoredDefinition`,
  `ValidatedDefinition`, `CurrentnessFamilyRequirement`, `SourceDocument`,
  `CapturedRevisions`, `ProjectionSource::load`,
  `ProjectionSourceSelection`, `ProjectionCurrentnessRequest::captured`,
  `CurrentnessCheck`, `LiveCurrentnessQuery`, `LiveCurrentnessObservation`,
  `validate_definition`, `validate_currentness_requirements`, `bind_sources`,
  `validate_selected_source_semantics`, `validate_request_currentness`, and
  the pair-validation call immediately after exact source binding.
- `crates/engine/src/projection/tests.rs` — private regression helpers and the
  fixed-input source-pair proof matrix.
- `crates/engine/src/projection/fixtures/source.json` — the retained
  one-source compatibility fixture.
- `crates/engine/src/projection/fixtures/snapshot-source-pair.json` — the one
  new private test-only fixed canonical source-pair fixture.

The primary commit may also contain this selector, its bounded implementation
proof records, and immutable v1.4 internal dispatches. The mechanical commit
may contain only the completed parent handoff, rebuilt ledger, and an exact
P3/P4 inventory transcription when required. No other path is admitted.

## Frozen private implementation contract

1. Decode a raw source envelope into one normalized private captured-family
   closure. Exactly one of legacy `captured_revisions` and non-empty canonical
   `captured_family_revisions` is valid. The normalized closure is family
   sorted and family unique; malformed, both, missing, empty, unsorted, or
   duplicate forms refuse before payload access.
2. Represent adapters as exact ref/fingerprint pairs and state identity as an
   optional private fingerprint. A legacy unpaired selection has no state
   fingerprint. Add private source dependencies only on the source envelope;
   every dependency has one role, one exact source pair, and one required
   state fingerprint.
3. Add a definition-declared, optional private source-pair requirement. The
   sole selected requirement binds `snapshot_current` to `snapshot_delta`
   through one `to_snapshot` dependency and requires current state identity.
   It runs after exact selector binding and before source semantics,
   currentness, disclosure, or payload access.
4. Currentness uses the selected current source's canonical closure. The
   fixed proof requires exactly `git`, `handbook`, `work` (`work_ledger` and
   `active_plan`), `session`, and `evidence`; each has the exact selector and
   adapter pair. Empty declared slots are valid for scalar families. A result
   contains exactly one check per family, with expected/captured/observed slot
   maps nested in the work check.
5. Both bound source selections, including the required current state
   fingerprint, remain in generic result provenance. The result remains
   immutable and always has `authority_effect: none`.

## Required proof and compatibility walls

- Retained one-source currentness fixture retains its success and refusal
  behavior.
- The fixed private compatible pair proves exact two-source provenance,
  five-family captured/live equality, typed disclosure and omission,
  redaction without an original-payload read, independently evaluated retained
  pointers, and `authority_effect: none`.
- Missing or duplicate roles, substituted exact pairs/state fingerprints,
  missing/duplicate/mispointed `to_snapshot`, invalid source closures,
  missing/duplicate/extra/substituted tuples, stale family or slot live
  observations (including session), and insufficient Resolution all refuse
  before result or unauthorized payload access.

## Explicit non-goals and stops

Do not add a public export, schema, dependency, configuration, adapter, CLI,
SDK, packet, Handoff/pipeline consumer, Snapshot Memory runtime serializer,
HCM-3.5/HCM-3.6 work, or Phase-3 claim. Stop on any public/API/dependency
need, unapproved path, authority ambiguity, required platform proof gap,
protected-path discrepancy, local-ref drift, or P1/P2 outside the permitted
causal cadence. GitNexus tooling is unavailable in this task environment, so
every required impact or change-detection result must be marked unavailable,
never GREEN.
