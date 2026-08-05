# HCM-3.2 implementation supplemental remediation

Parent: `20260805T014559Z--HCM-3-2--context-resolution-finalization`

Outcome: `hcm-3.2-context-resolution-finalization-whole-slice`

The different-fresh closure for subject
`sha256:ee9ccc70e5a896241056054fe7f07f2addd74668fdc022b0ee0fd16cfefe0196`
returned three unchanged-scope P2 findings. They retain their stable source IDs:

- `HCM32-JCS-IMPL-DISC-005`: the open quarantine record was
  self-refingerprinted and therefore did not retain an independent original
  identity anchor;
- `HCM32-JCS-IMPL-DISC-004`: the real zero-candidate recovery branch and exact
  multiple-candidate classifier lacked executable assertions;
- `HCM32-JCS-IMPL-DISC-006`: canonical completion/gate language was
  unconditional while different-fresh closure was still pending.

This is causal supplemental 1 for the implementation stage. It does not reopen
discovery, change the selector, or expand the production/test/risk boundary.

## Consolidated repair

The generic lineage owner now writes an append-only quarantine anchor before
the mutable diagnostic record. The anchor binds the transition fingerprint to
the original open-record fingerprint. Inventory validation requires every open
record to reproduce that exact anchor before candidate admission, promotion,
completion, reconciliation, or operational recovery. An anchor left before an
open record is inert and can be completed by exact replay; a missing, changed,
duplicate, unsafe, or mismatched anchor fails closed.

The public wrong-candidate test now also calls the lock-bound admission seam
with the same self-refingerprinted substitute identity. Before repair that call
returned success; after repair it refuses before canonical, promotion-journal,
or quarantine mutation. Recovery proof removes the candidate inventory and
observes `candidate_missing` through the real cold-load path. The exact
cardinality classifier separately asserts that two candidates produce
`candidate_ambiguous` and no candidate identity.

Canonical control statements are conditional on CLEAN in
`20260805T073000Z--HCM-3-2--finalization-implementation-supplemental-1`.
`FINDINGS` or `BLOCKED` leaves HCM-3.2 and its exact gate subsets incomplete;
the files require no post-review truth rewrite.

## Observed proof wall

- RED: `different_valid_quarantine_candidate_refuses_before_publication_mutation`
  failed because the lock-bound admission seam accepted the self-refingerprinted
  substituted record.
- GREEN: the same test passed `1/1` in `117.48s` after the anchor repair.
- GREEN: the expanded real recovery classification test passed `1/1` in
  `448.71s`, including `candidate_missing`.
- GREEN: `zero_and_multiple_candidates_refuse_exactly` passed `1/1` and asserts
  `candidate_missing` and `candidate_ambiguous` exactly.
- GREEN: the complete Context Resolution kernel passed `31/31` in `600.05s`.
- GREEN: generic lineage passed `55/55` in `391.20s`.
- GREEN: `handbook-engine` all-target/all-feature tests exited `0` in
  `1288.3s`.
- GREEN: full workspace all-target/all-feature tests exited `0` in `1622.6s`.
- GREEN: workspace check, strict all-target/all-feature Clippy, formatting, and
  whitespace all exited `0`.

FTS remains unavailable and is not represented as GREEN. GitNexus impact for
the changed anchor inventory/recording seams was LOW (15 and 3 impacted
symbols, two and zero affected processes); the already selected recovery gap
seam remained LOW (8 impacted symbols, two affected processes). The final
production additions remain inside the reviewed ceilings: lineage `561/600`,
mutation `173/500`, Context Resolution `1286/1300`, aggregate `2020/2400`, and
three production paths.

The exact supplemental-1 reviewer returned `FINDINGS`, retaining only
`HCM32-JCS-IMPL-DISC-005`: an anchor-only crash could still bypass admission,
completion, and reconciliation because absent-open success preceded inventory
validation, and completion scratch was removed before identity validation.
Findings `004` and `006` closed. This verdict authorizes only the directly
causal supplemental-2 repair recorded in
`20260805T085700Z--finalization-implementation-supplemental-2-remediation.md`.
