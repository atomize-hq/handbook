# HCM-2.2 Atomic Stage Review 2 Remediation Proof

## Review result

Different fresh isolated read-only reviewer
`/root/hcm_2_2_atomic_stage_review_2` admitted all 22 paths and aggregate
`sha256:34ac6911c66179f56e43adcc5603d049841f65aedcde9303a25e1719fd2eb1aa`
from immutable dispatch
[`../../../handoffs/dispatches/20260720T204719Z--HCM-2-2--atomic-stage-authority-repair-planning-review-2.json`](../../../handoffs/dispatches/20260720T204719Z--HCM-2-2--atomic-stage-authority-repair-planning-review-2.json).
It returned `CHANGES_REQUIRED` with one Required finding,
`HCM-2.2-ASAR-PR2-001`. The parent accepted the finding without waiver.

## Finding and bounded repair

The primary promotion-intent contract, schema, and vector key the pending
journal by the intent-bound `transaction_id`, which is distinct from the
promotion record's `promotion_id`. One stale placeholder in the late promotion-
protocol expansion nevertheless used `<promotion-id>.pending/`. The repair
changes only that placeholder to `<transaction_id>.pending/`.

The machine vector now states explicitly that writer, discovery, recovery, and
finalization derive the pending directory solely from validated
`transaction_id`; `promotion_id` is forbidden as a journal key. The existing
amendment positive intentionally has unequal transaction and promotion IDs, and
the negative set now refuses every promotion-ID or other-value pending-path
derivation. Plan and checklist require the same assertion.

Intent `1.2`'s data schema, transaction/promotion identities, expected
fingerprint, 7,607-byte persisted document, 72-byte marker, and expected
transaction-keyed pending directory remain unchanged. No earlier dispatch,
proof, handoff, review, released record, Rust/test file, or external dirty-
worktree snapshot was modified.

## Remediation validation

Targeted executable assertions prove:

- the promotion-protocol expansion contains
  `.handbook/state/transactions/promotions/<transaction_id>.pending/` and no
  promotion-ID-keyed pending path;
- the positive vector's `transaction_id` and `promotion_id` are unequal;
- deriving the expected pending path from that exact `transaction_id`
  reproduces the frozen positive value;
- the vector has an explicit writer/discovery/recovery/finalization identity
  rule and a distinct-ID negative; and
- `git diff --check`, unchanged intent-schema bytes, and zero modified pre-
  existing immutable evidence checks pass.

The parent must replay the complete proof wall, create a new exact subject
manifest, and use another different fresh isolated read-only reviewer. This
proof records no clean review and grants no implementation or HCM-2.3
authority.
