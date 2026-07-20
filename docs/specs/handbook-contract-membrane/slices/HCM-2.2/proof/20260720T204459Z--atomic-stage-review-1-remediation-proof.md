# HCM-2.2 Atomic Stage Review 1 Remediation Proof

## Review result

Fresh isolated read-only reviewer `/root/hcm_2_2_atomic_stage_review_1`
admitted all 21 paths and aggregate
`sha256:2fab38a467cb4831fe0a780430b245db0636d537a3c573ba8677f5b071a140f8`
from immutable dispatch
[`../../../handoffs/dispatches/20260720T202209Z--HCM-2-2--atomic-stage-authority-repair-planning-review-1.json`](../../../handoffs/dispatches/20260720T202209Z--HCM-2-2--atomic-stage-authority-repair-planning-review-1.json).
It returned `CHANGES_REQUIRED` with Required findings
`HCM-2.2-ASAR-PR1-001` and `HCM-2.2-ASAR-PR1-002` plus Nit finding
`HCM-2.2-ASAR-PR1-003`. The parent accepted all three without waiver.

## Findings and bounded repair

1. A late duplicate promotion-protocol expansion still created
   `intent.tmp` inside the authoritative pending journal and admitted its
   partial bytes during pre-intent recovery. The repaired expansion now uses
   only the already-selected same-filesystem non-authoritative
   `.intent-staging/<32hex>.intent` whole-file protocol: retain complete JCS+LF
   intent bytes, create-new/write/file-fsync/close, bounded/no-follow exact
   reverify, scratch-directory fsync, create+fsync an empty pending directory,
   atomic rename-no-replace directly to `intent.json`, ordered pending/scratch/
   transaction-parent fsyncs, and final exact reverify. Promotion recovery now
   admits only the exact empty pending directory before intent; `intent.tmp`, a
   partial `intent.json`, any other name, or any other incomplete state
   preserves evidence and refuses. The separate registry and standalone-
   lifecycle journal protocols remain unchanged.
2. The vector named `S0` but fault enumeration began at `S1`. The repaired
   vector freezes `S0` as retained complete expected bytes/bindings before
   scratch creation, with no scratch and an absent pending stage. Its crash
   rule, fault enumeration, all three purpose vectors, SPEC, plan, and checklist
   now require the exact 36-pair Cartesian product
   `{canonical, promotion-record, lifecycle-transition} × {S0,...,S11}` plus
   every bounded `S2` scratch-prefix fixture. Missing or duplicate pairs fail
   conformance.
3. The research note's two stale status-surface filenames are corrected to the
   existing `03-seam-crosswalk.md` and `04-phase-slice-map.md`. Its remediated
   SHA-256 is
   `4fa17109c8270038b6366fb5e5af3559bf95e4b6ae7b7ec9f3c041f1a2e2dd55`.

No earlier dispatch, proof, handoff, review, released record, intent schema,
Rust file, test file, or external dirty-worktree snapshot was modified.

## Remediation validation

Targeted executable assertions prove:

- the promotion-protocol section contains the exact `.intent-staging/`
  publication and empty-only pre-intent recovery rules;
- that section has no create/write path for pending `intent.tmp` and never
  admits partial promotion-intent bytes;
- the machine vector independently contains 12 unique ordered boundary labels,
  three unique ordered purpose suffixes, and exactly 36 unique purpose/boundary
  pairs;
- `S0` is explicit in the crash rule, fault enumeration, and each purpose
  vector;
- both corrected mutable status paths exist and both stale names are absent;
- normal handoff validation accepts the new immutable Review 1 dispatch and
  exact existing ledger; and
- `git diff --check` passes with zero modified pre-existing immutable handoff,
  dispatch, or proof paths.

The parent must replay the complete proof wall, create a new exact subject
manifest, and use a different fresh isolated read-only reviewer. This proof
records no clean review and grants no implementation or HCM-2.3 authority.
