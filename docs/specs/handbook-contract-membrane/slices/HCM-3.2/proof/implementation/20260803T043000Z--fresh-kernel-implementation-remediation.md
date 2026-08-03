# HCM-3.2 fresh kernel implementation remediation proof

Scope: dispatch `20260803T034500Z--HCM-3-2--fresh-implementation-remediation`.
No selector, `artifact_mutation.rs`, dependency, public signature, or production
path outside the frozen four-path subject changed.

## Finding dispositions

- `HCM32-FRESH-IMPL-DISC-001` — remediated. The authority resolver now refuses
  only an empty eligible set and supplies the complete sorted eligible set to
  GetAssertion. A real two-credential committed-registry test proves both IDs
  are in the request and one eligible credential may be selected; uncovered
  (zero) and unknown response selection refuse.
- `HCM32-FRESH-IMPL-DISC-002` — remediated. A superseded disposition retains
  its exact superseding request. Registry admission requires that request to be
  distinct and already admitted with the same ref/fingerprint. Self, unknown,
  and stale requests refuse; the exact second admitted request succeeds.
- `HCM32-FRESH-IMPL-DISC-003` — remediated. Promotion disposition admission
  compares its recorded `approving_authority_ref` with the authenticated
  authority binding's approving mapping. Mismatch and empty refuse; the exact
  `work.approver` mapping succeeds.
- `HCM32-FRESH-IMPL-DISC-004` — remediated. Behavior tests now call root and
  child resolution, independently narrow all six dimensions, exercise allow,
  deny-overlap and indeterminate mutation outcomes, typed memory/validation
  outcomes, escalation request/disposition currentness, promotion request and
  disposition authority, terminal admission, and stack empty/upper bounds,
  duplicate IDs, noncontiguous ranks, and adjacent-level widening.
- `HCM32-FRESH-IMPL-DISC-005` — confirmed immutable blocker, not waived. The
  selected schema reports exactly 53 leaves. The existing generic mutation
  owner rejects a distinct output closure outside `1..=15`
  (`artifact_mutation.rs:1643-1646`), while the lineage owner independently
  requires `1..=15` outputs and coverage entries
  (`artifact_lineage_store.rs:617-618`, `1136-1139`). The inspected existing
  repository APIs (`ArtifactRepositoryV1::evaluate_intake` and
  `evaluate_intake_document`) can evaluate the 53 coverage submissions but do
  not persist owner lineage; persistence necessarily crosses the immutable
  15-output mutation-owner contract. Therefore the seeded canonical binding
  bytes remain test setup, not owner-produced lineage. Producing valid lineage
  requires selector/owner-contract or production-path expansion forbidden by
  this dispatch.

## RED/GREEN evidence

The pre-fix mutation probe failed all three regression tests as intended:

- complete credential allow-list: exit 101 at the old exactly-one guard;
- superseding-request currentness: exit 101 because self-supersession returned
  `Ok(true)`;
- promotion approving authority: exit 101 because a mismatched ref returned
  `Ok(true)`.

After restoring the remediation, the four focused kernel behavior tests passed
4/4 and `context_resolution_stack` passed 4/4. The initial complete kernel run
passed 20/21 and exposed only a test counter fixture that reused retained count
1; after advancing the test assertion to count 2, that exact failed test passed
1/1. Thus every one of the 21 enumerated kernel tests has passing evidence,
without claiming a fabricated single-run count.

Production declaration count remains 79 (no declaration added), within the 80
ceiling. Production paths remain 4/4, test families remain 3/3,
and hand-written production remains below 2,000 lines. Finding 005 is a true
top-level stop for causal closure.
