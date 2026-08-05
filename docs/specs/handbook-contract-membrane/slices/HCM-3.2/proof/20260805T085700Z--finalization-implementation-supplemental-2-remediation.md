# HCM-3.2 implementation supplemental-2 remediation

Parent: `20260805T014559Z--HCM-3-2--context-resolution-finalization`

Outcome: `hcm-3.2-context-resolution-finalization-whole-slice`

Causal supplemental 1 reviewed exact subject
`sha256:c263360d0d6dd0cb0c37564ca536e89fb7fb5c29f2a6a726730bbf26f929f640`
and returned `FINDINGS`. It closed stable findings
`HCM32-JCS-IMPL-DISC-004` and `006`, and retained only
`HCM32-JCS-IMPL-DISC-005` P2. The retained issue was directly unmasked by the
preceding anchor repair: a crash after the durable anchor write but before the
open-record write left an anchor-only state, while admission, completion, and
reconciliation returned success on an absent open path before inventory
validation. Inventory also removed completion scratch before validating the
open-record/anchor binding.

This is causal supplemental 2 for the unchanged implementation subject. It
does not reopen discovery, alter stable finding identity, or expand selector,
production-path, API, dependency, platform, transport, or risk authority.

## Exact repair

Quarantine inventory now reads and validates the anchor inventory even when
the quarantine record root is absent. Every anchor requires its exact open
source record; every committed receipt requires that same open record; and
admission, completion, and reconciliation validate the full inventory before
any absent-open success. Anchor-only and committed-only states therefore
refuse without authorizing publication, current use, or completion.

Completion scratch entries are classified and checked for safe names and file
identity, but are removed only after anchor/open/committed identity validation
succeeds. An invalid or self-refingerprinted open record leaves its scratch
bytes unchanged. The sole recovery exception is the original recording seam:
an exact retry whose open-record bytes reproduce the already durable anchor
may reconstruct the missing open record, after which ordinary inventory
validation runs. A different record cannot replace or reinterpret the anchor.

## Adversarial evidence

- `quarantine_anchor_only_and_committed_only_states_fail_closed` passed `1/1`
  in `110.46s`. It proves anchor-only admission, completion, and reconciliation
  refuse without mutation; exact anchor-bound recording reconstructs the open
  record; and committed-only admission and reconciliation refuse without
  mutation.
- `different_valid_quarantine_candidate_refuses_before_publication_mutation`
  passed `1/1` in `117.61s` with a partial completion scratch present. The
  substituted self-refingerprinted open record refuses and the scratch remains
  byte-identical.
- Complete Context Resolution kernel passed `32/32` in `595.41s`.
- Complete generic lineage passed `55/55` in `258.51s`, including unchanged
  non-HCM markerless behavior.
- `handbook-engine` all-target/all-feature tests exited `0` in `1280.9s`.
- Workspace all-target/all-feature tests exited `0` in `1630.2s`.
- Workspace all-target/all-feature check exited `0` in `5.43s`.
- Strict workspace all-target/all-feature Clippy exited `0` in `8.64s` after
  one non-semantic statement-boundary line split.
- Formatting and whitespace checks exited `0`.

The final production additions remain within the reviewed ceilings: lineage
`577/600`, mutation `173/500`, Context Resolution `1286/1300`, aggregate
`2036/2400`, and three production paths. FTS remains unavailable and is not
represented as GREEN. Fresh impact was exact LOW for the indexed inventory
helper and completion seam; the two new compact absent-open methods were not
present in the index and their direct caller boundary was therefore recorded
as unavailable rather than GREEN. They remain inside the already warned and
reviewed Context Resolution lineage-recovery seam.

Final implementation acceptance is owned only by the different-fresh reviewer
for exact dispatch
`20260805T090000Z--HCM-3-2--finalization-implementation-supplemental-2`.
`FINDINGS` or `BLOCKED` leaves `HCM32-JCS-IMPL-DISC-005` open. No later
implementation cycle is authorized after CLEAN.
