# HCM-2.2 Atomic Stage Review 4 Remediation Proof

## Review result

Fourth different fresh isolated read-only reviewer
`/root/hcm_2_2_atomic_stage_review_4` admitted all 24 paths and aggregate
`sha256:d4a8560f3bf48c3d4848be5c9ade40964a1a551f5eb4bcd8bef24a2bdaa61374`
from immutable dispatch
[`../../../handoffs/dispatches/20260720T212832Z--HCM-2-2--atomic-stage-authority-repair-planning-review-4.json`](../../../handoffs/dispatches/20260720T212832Z--HCM-2-2--atomic-stage-authority-repair-planning-review-4.json).
It returned `CHANGES_REQUIRED` with Required finding
`HCM-2.2-ASAR-PR4-001` and Nit finding `HCM-2.2-ASAR-PR4-002`. The parent
accepted both without waiver.

## Findings and bounded repair

1. The prior terminal-destination vector exposed one generic four-state axis,
   but SPEC/checklist explicitly crossed it only for `.committed`. The repaired
   contract defines independent `committed_destination` and
   `rolled_back_destination` axes, each exactly
   `{absent, exact_pre_existing, mismatching, unsafe}`. Published
   `committed`/`rolled-back` under `.pending` cross all four corresponding
   destination states. Only absent permits terminal rename-no-replace; exact,
   mismatch, unsafe, or simultaneous suffix preserves the complete pending
   journal and refuses without replacement or cleanup. The machine state axes,
   recovery rule, negatives, SPEC, plan, and checklist now state this
   symmetrically.
2. The research note cited mutable current SPEC/plan/checklist line ranges for
   the superseded exact-prefix grammar, so later repair made those citations
   semantically stale. It now cites immutable pre-repair commit
   `cfd954c3b25f6d27949c8d6c2fab42195fb251bb`, gives the exact supporting line
   ranges at that commit, provides the `git show <commit>:<path>` reproduction
   method, and links the immutable 19-path Review 6 manifest. Claims about dirty
   runtime behavior are explicitly identified as verified external-snapshot
   evidence instead of pretending the clean planning worktree contains those
   uncommitted sources.

Intent `1.2`, all semantic identities, scratch publication, terminal fsync
replay, and immutable prior evidence remain unchanged. No Rust/test file or
external snapshot byte was modified.

## Remediation validation

Targeted executable assertions prove:

- two independent destination axes each have exactly four unique values,
  yielding eight unique terminal-destination pairs;
- exactly the two absent-destination pairs permit terminal rename;
- `.committed` and `.rolled-back` each have explicit collision/unsafe
  negatives and participate in the machine state axes;
- the immutable baseline object reproduces the cited partial-stage and stop
  passages; and
- JSON parse, `git diff --check`, and unchanged intent-schema-byte checks pass.

The parent must replay the complete proof wall, create a new exact subject
manifest, and use another different fresh isolated read-only reviewer. This
proof records no clean review and grants no implementation or HCM-2.3
authority.
