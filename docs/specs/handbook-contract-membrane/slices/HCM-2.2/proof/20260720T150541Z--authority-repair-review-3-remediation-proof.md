# HCM-2.2 Authority-Repair Review 3 Remediation Proof

## Review result

Different fresh isolated reviewer `/root/hcm_2_2_authority_repair_review_3`
admitted all 16 paths and aggregate
`sha256:3463c1840a60e1db4deda3ee69df298fb2e40dacaf06d661d2ec088e67fefc1c`
from immutable dispatch
[`../../../handoffs/dispatches/20260720T143935Z--HCM-2-2--authority-repair-planning-review-3.json`](../../../handoffs/dispatches/20260720T143935Z--HCM-2-2--authority-repair-planning-review-3.json).
It returned `CHANGES_REQUIRED` with one Required finding,
`HCM-2.2-ARPR3-001`, and no Critical, Optional, or Nit findings. The finding was
accepted without waiver.

## Finding and repair

Rollback recovery had been described as one recovery-table action rather than
as a crashable writer. A crash during cleanup, during `rolled-back.tmp`
publication, after `rolled-back` publication while the suffix was still
`.pending`, after the terminal rename, or before the transaction-parent fsync
could therefore produce a writer-reachable state outside the declared
`W0`-`W15` domain and fall into mismatch.

The repaired contract now freezes cumulative recovery boundaries `R0`-`R9`:

1. revalidate an exact `W3`-`W9` rollback origin;
2. finish and fsync the exact amendment snapshot;
3. remove exactly `prepared.tmp`, `prepared`, `lifecycle-transition.new`,
   `promotion-record.new`, and `canonical.new`, in that order, with a no-follow
   reverify and pending-directory fsync after each present-name unlink;
4. admit and append-complete every exact zero-through-72-byte
   `rolled-back.tmp` prefix;
5. publish and fsync the exact rollback marker;
6. recognize and terminalize the exact published rollback payload still under
   `.pending`; and
7. rename to `.rolled-back`, fsync the transaction parent, and repeat that
   parent fsync on every exact terminal observation before returning.

The admitted cleanup family is constructed, not guessed: it is the union of
every exact `W3`-`W9` rollback origin after exact snapshot completion and every
prefix of the fixed cleanup list. Removing forward marker state before staged
files preserves marker causality. Every state outside this construction stays
an evidence-preserving mismatch. The SPEC recovery partition now has separate,
disjoint rows for cleanup, partial rollback-marker continuation, and published
pending rollback finalization. The base contract, vector, implementation plan,
and checklist freeze the same rules. No prior dispatch or proof was rewritten.

## Remediation validation

- JSON parsing and Draft 2020-12 schema/vector validation remain required and
  the unchanged intent semantic preimage/fingerprint/document/marker values
  must reproduce exactly;
- the vector freezes the five-name cleanup order, `R0`-`R9`, all marker-prefix
  lengths, pending rollback finalization, terminal parent-fsync replay, and
  mismatch negatives;
- conformance enumerates every rollback origin, amendment snapshot prefix,
  cleanup cursor, rollback-marker prefix, marker publication, terminal rename,
  and parent-fsync replay;
- fault injection stops after every rollback reverify, unlink, directory fsync,
  marker write/fsync/rename, terminal rename, and parent fsync; retry must select
  one row and reach the exact immutable `.rolled-back` terminal set;
- the nine non-catch recovery predicates must remain pairwise disjoint and every
  non-writer or crossed-suffix state must preserve evidence and refuse;
- all mutable-subject local Markdown targets, `git diff --check`, and the
  documentation-only scope invariant must pass.

The next review must use another different fresh isolated read-only reviewer
over a new exact complete-subject manifest. This proof grants no implementation
or HCM-2.3 authority.
