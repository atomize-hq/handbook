# HCM-2.2 promotion recovery: authentic byte-source options

**Status:** research note only; non-authoritative. HCM-2.2 implementation and
HCM-2.3 remain stopped until a repaired planning subject receives a fresh
`CLEAN` review.

## Finding

The implementation stop is correct. The approved transaction grammar permits a
crash during `W5`-`W7` to leave `canonical.new`, `promotion-record.new`, or
`lifecycle-transition.new` as an exact prefix. Recovery may unlink those files
only after proving that they are prefixes of the exact intended bytes
in the immutable pre-repair authority at commit
`cfd954c3b25f6d27949c8d6c2fab42195fb251bb`: SPEC lines 288-320 freeze the
partial-stage writer grammar, lines 322-368 freeze rollback `R0`-`R6`, and
lines 408-412 preserve every unprovable journal. At the same commit, plan lines
245-276 and checklist lines 120-137 repeat those obligations. Reproduce this
evidence with `git show <commit>:<path>`; the exact 19-path byte manifest and
aggregate are preserved by immutable
[`authority-repair-planning-review-6`](../../../handoffs/dispatches/20260720T154437Z--HCM-2-2--authority-repair-planning-review-6.json).

Intent `1.2` does not contain any output's complete bytes or authenticated
prefix/chunk commitments. It contains only:

- canonical semantic fingerprint, whole-document SHA-256, and byte length; and
- each record's ref, semantic fingerprint, whole-document SHA-256, and byte
  length.

That closed shape is frozen by
[the schema, lines 474-551](../contracts/promotion-transaction-intent-1.2.0.schema.json#L474-L551),
[the vector, lines 48-64](../contracts/promotion-transaction-intent-vectors-v1.0.json#L48-L64),
and the exact immutable pre-repair manifest above. A whole-document
digest and total length can verify complete candidate bytes, but they do not
supply the expected bytes needed to compare a shorter file with the intended
prefix.

The lifecycle transition makes the gap conclusive. Its semantic fingerprint
explicitly excludes `transitioned_at_utc`, while the timestamp remains in the
persisted document
([runtime vector, lines 726-775](../contracts/runtime-record-fingerprint-vectors-v1.0.json#L726-L775)).
The verified external implementation snapshot independently exhibited the same
exclusion and serialized exact record bytes. Thus byte-distinct
transition documents can share the same ref and semantic fingerprint. If the
final record is absent after a crash, intent metadata cannot identify the
chosen timestamp or authenticate an arbitrary `lifecycle-transition.new`
prefix.

The verified external implementation snapshot showed all three exact outputs
retained only in process memory before direct staged writes. A crash during a
write can lose that in-process source while leaving a partial file. Therefore the
approved `R4`-`R6` deletion rules cannot be implemented without relaxed
validation or unauthorized evidence deletion. The immutable pre-repair SPEC at
commit `cfd954c3b25f6d27949c8d6c2fab42195fb251bb` requires a stop whenever
persistence or recovery remains implicit (lines 431-438).

## Options

### Option A — atomic whole-file scratch publication (recommended)

Construct and verify each complete output in a non-authoritative sibling
scratch namespace, then atomically rename-no-replace the complete file into its
named `.pending` stage and fsync the pending directory. This mirrors the already
approved `intent.json` protocol
([SPEC, lines 270-286](../SPEC.md#L270-L286)).

The revised `W5`-`W7` state for each output becomes binary: the pending stage is
absent or exact complete bytes. A crash may orphan a scratch file outside the
journal scan, but it cannot expose a partial authoritative stage. Recovery can
then authenticate a present stage using the existing document SHA-256 and
length, and rollback can remove it after exact verification.

This option is preferred because it:

- removes the unauthenticatable state instead of expanding recovery authority;
- keeps intent `1.2`'s closed data schema unchanged;
- follows a whole-file publication mechanism already specified and reviewed;
- reduces the Cartesian recovery state space and fault matrix; and
- avoids embedding potentially large canonical and record bytes, prefix tables,
  or chunk-tree rules in the intent.

The repaired authority must still define the scratch root and random-name
grammar, size bounds, no-follow/create-new rules, same-filesystem requirement,
write/fsync/close/reopen/exact-byte verification, directory fsync order,
rename-no-replace behavior, crash states, and the fact that scratch orphans are
never journal authority or recovery evidence. Garbage collection should remain
separately authorized.

### Option B — authenticated complete bytes or byte commitments

Retain partial files in `.pending`, but add an authentic recovery source for
every output. The direct form stores the complete bytes; a commitment form
stores enough domain-separated chunk or prefix commitments to authenticate
every writer-reachable partial length.

This can preserve the existing W5-W7 grammar, but it is not recommended. It
requires a new intent schema/version, new fingerprint and size vectors,
canonical chunking and final-short-chunk rules, exact prefix-length admission,
larger bounded persistence, and substantially more recovery and corruption
proof. A single whole-document SHA-256 is not a prefix commitment. Merely adding
`transitioned_at_utc` is also insufficient because it repairs only the known
lifecycle reconstruction gap and still leaves the byte-source contract
implicit.

## Exact authority-repair surface

Use a new documentation-only authority-repair packet. Do not edit immutable
handoffs, dispatches, reviews, or proof records. The complete fresh-review
subject should include:

1. `slices/HCM-2.2/SPEC.md`: replace the W5-W7 partial-stage grammar, the R0/R4-
   R6 prefix predicates, recovery partition, Cartesian axes, writer/recovery
   fault boundaries, and mismatch wording with the selected absent-or-exact
   atomic-publication model. Preserve the independent `canonical.old` prefix
   rule: amendment recovery has an authentic source in the retained exact old
   canonical target.
2. `slices/HCM-2.2/contracts/promotion-transaction-intent-vectors-v1.0.json`:
   revise `writer_state_model`, stage axes, negative vectors, rollback cleanup,
   and fault enumeration. Add positive and crash vectors for each scratch write,
   verification, rename, and directory-fsync boundary.
3. `slices/HCM-2.2/contracts/promotion-transaction-intent-1.2.0.schema.json`:
   under Option A, retain it byte-for-byte and record that no identity field is
   required; revalidate its published hash and vectors. Under Option B, version
   it additively and regenerate every dependent vector—never mutate `1.2` in
   place.
4. `slices/HCM-2.2/tasks/plan.md` and `tasks/todo.md`: retain the stop record,
   add the selected repair and RED/fault/conformance proof, and keep
   implementation unauthorized until planning review is `CLEAN`.
5. The pack's mutable status and proof-ledger surfaces (`00-README.md`,
   `03-seam-crosswalk.md`, `04-phase-slice-map.md`, and
   `06-proof-and-regression-ledger.md`) only where needed to keep HCM-2.2
   explicitly stopped during repair. Add new escalation/decision/review/handoff
   records through the existing protocol rather than changing earlier records.

The fresh reviewer must review the complete updated persistence and recovery
subject, not only the changed W5-W7 table rows. HCM-2.2 must not be described as
complete, and HCM-2.3 must remain unauthorized.

## Preserving and resuming the dirty implementation

The current implementation is useful checkpoint evidence but is not a landing
candidate under the ambiguous contract.

1. Leave HEAD at
   `4e164061e18da17cc24d576a26800ab1ecce69c2`; do not reset, amend, or discard
   the dirty Rust and test changes.
2. Inventory the exact dirty paths and preserve a recoverable local snapshot or
   patch before any branch/worktree operation. Do not include secrets or
   generated build output.
3. Prepare and review the authority-only repair in a separate clean worktree or
   branch from that HEAD. Import the stop fact and this research note, but do not
   import the Rust diff into the planning review subject.
4. After the authority repair is committed and review-clean, retain the original
   dirty worktree as checkpoint evidence. Resume from the repaired authority
   branch and reapply the partial implementation selectively, hunk by hunk,
   rather than treating the old diff as pre-approved.
5. Reconcile `tasks/todo.md` in favor of the newer reviewed authority while
   preserving the historical stop fact. Re-run GitNexus impact analysis before
   every affected symbol edit, including the previously accepted HIGH
   lifecycle-anchor scope, and run change detection before any eventual commit.
6. Add the new scratch-publication RED tests first, then finish the W0-W15 and
   R0-R9 implementation and exhaustive proof. Only after full proof and a fresh
   complete-subject `CLEAN` implementation review may the scoped implementation
   and separate closeout commits be created.

This procedure preserves the green partial work without allowing it to acquire
authority retroactively.
