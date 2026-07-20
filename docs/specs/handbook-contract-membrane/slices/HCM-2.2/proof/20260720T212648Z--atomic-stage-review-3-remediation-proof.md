# HCM-2.2 Atomic Stage Review 3 Remediation Proof

## Review result

Third different fresh isolated read-only reviewer
`/root/hcm_2_2_atomic_stage_review_3` admitted all 23 paths and aggregate
`sha256:029e3d5bb50102f26ca11bf3729a112cbf265f021a139fa109aaf2d91498f12e`
from immutable dispatch
[`../../../handoffs/dispatches/20260720T210619Z--HCM-2-2--atomic-stage-authority-repair-planning-review-3.json`](../../../handoffs/dispatches/20260720T210619Z--HCM-2-2--atomic-stage-authority-repair-planning-review-3.json).
It returned `CHANGES_REQUIRED` with one Required finding,
`HCM-2.2-ASAR-PR3-001`. The parent accepted the finding without waiver.

## Finding and bounded repair

Marker publication already required pending-directory fsync after atomic marker
rename, but recovery could not observe whether that fsync completed. The
published-`committed` and published-`rolled-back` predicates proceeded to
terminal directory rename without explicitly replaying the child-directory
fsync. The repair now requires every observation of either exact terminal
marker under `.pending` to:

1. repeat the pending-directory fsync;
2. exact-revalidate the complete committed or rollback terminal payload; and
3. only then rename-no-replace the pending directory to its terminal suffix,
   fsync the transaction parent, and reverify the terminal set.

The rule is frozen symmetrically in the marker contract, `W14`, `R8`, both
recovery predicates, the suffix partition, machine vector, plan, checklist, and
control-pack summary. The fault grammar injects for both terminal markers after
marker rename but before the writer's pending-directory fsync, after that fsync,
after recovery's replayed pending-directory fsync, and after terminal-payload
revalidation. Every retry repeats the child-directory fsync and converges to
the exact immutable terminal set.

Intent `1.2`, all identity values, output-scratch grammar, journal names, and
terminal payload bytes remain unchanged. No earlier dispatch, proof, handoff,
review, released record, Rust/test file, or external dirty-worktree snapshot
was modified.

## Remediation validation

Targeted executable assertions prove:

- both machine finalization rules place the repeated pending-directory fsync
  and exact payload revalidation before terminal rename;
- the exact terminal-marker fault product contains two markers by four replay
  boundaries, producing eight unique pairs;
- SPEC freezes the same `W14`, `R8`, committed-predicate, rollback-predicate,
  and retry ordering; and
- `git diff --check` and unchanged intent-schema-byte checks pass.

The parent must replay the complete proof wall, create a new exact subject
manifest, and use another different fresh isolated read-only reviewer. This
proof records no clean review and grants no implementation or HCM-2.3
authority.
