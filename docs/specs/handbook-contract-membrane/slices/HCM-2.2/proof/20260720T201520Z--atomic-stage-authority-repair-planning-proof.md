# HCM-2.2 Atomic Stage Authority-Repair Planning Proof

## Scope and authority

This additive proof belongs to user-selected escalation `HCM-2.2-ESC-002`.
Entry HEAD is `4e164061e18da17cc24d576a26800ab1ecce69c2`. The earlier
review-clean `HCM-2.2-ESC-001` Option 1 subject remains authority for candidate
`1.2`, lifecycle-validation-result `1.0`, promotion currentness, and promotion-
intent `1.2` except where this repair replaces the impossible `W5`-`W7`
new-output partial-stage grammar. This packet is documentation-only, changes no
Rust or test byte, does not resume implementation, and grants no HCM-2.3
authority.

The authentic-byte-source analysis is retained as non-authoritative decision
input in
[`../research/promotion-recovery-authentic-byte-source-options.md`](../research/promotion-recovery-authentic-byte-source-options.md).
It proves that intent `1.2` authenticates only a complete output document, its
SHA-256, and its byte length. In particular, the lifecycle transition's audit-
only timestamp is intentionally excluded from the semantic record fingerprint.
Recovery after process-memory loss therefore cannot authenticate or complete
an arbitrary partial pending new-output stage.

## Preserved implementation snapshot

Before this authority repair, the original dirty implementation worktree was
inventoried at the exact entry HEAD and preserved outside the repository:

- archive:
  `HCM-2.2-implementation-snapshot-20260720T195957Z.zip`;
- archive SHA-256:
  `29a7863787dc78902a2eb1dd948a2fbc9830b94a85fead40283a8c54d603cb9e`;
- manifest:
  `HCM-2.2-implementation-snapshot-20260720T195957Z.manifest.md`;
- inventory: 13 modified/untracked paths and zero deletions; and
- restore proof: all 13 files extracted to a separate verification directory
  and each extracted SHA-256 equalled the corresponding live dirty-worktree
  byte sequence.

The original worktree remained dirty and unchanged after capture. The archive,
manifest, extracted verification copy, checkpoint commits, and live dirty bytes
are recoverable evidence only. They are not staged, committed, imported into
this clean authority-repair worktree, or treated as correctness authority.

## Frozen repair

The selected repair keeps
[`../contracts/promotion-transaction-intent-1.2.0.schema.json`](../contracts/promotion-transaction-intent-1.2.0.schema.json)
byte-for-byte unchanged. It introduces no intent field, fallback, dual read, or
new semantic identity. It freezes:

1. sibling scratch root
   `.handbook/state/transactions/promotions/.output-staging/` on the same
   filesystem as the pending journal;
2. one independent engine-random 128-bit lowercase-hex create-new scratch name
   per output, with exact suffix `.canonical`, `.promotion-record`, or
   `.lifecycle-transition`;
3. the ordered `S0`-`S11` retained-bytes, scratch create/write, file-fsync/
   close, bounded no-follow reopen, exact verification, scratch-directory
   fsync, atomic rename-no-replace, pending/scratch/transaction-parent
   directory-fsync, and pending-stage reverify boundaries;
4. exact retained-byte plus intent document hash/length verification for every
   output, canonical fingerprint verification for canonical bytes, and closed
   schema/ref/semantic-fingerprint/full-intent/currentness verification for
   promotion and lifecycle records;
5. authoritative `canonical.new`, `promotion-record.new`, and
   `lifecycle-transition.new` axes exactly `{absent, exact, mismatch}`;
6. partial bytes exclusively in non-authoritative scratch, which selected
   readers and recovery never scan, classify, delete, compare, or use to derive
   authority;
7. preservation/refusal for any partial pending new-output stage, even a
   correct prefix; and
8. every bounded scratch prefix plus every `S0`-`S11` crash boundary for each
   output purpose, with pending-stage absence before publication and only
   absent-or-exact authoritative state after atomic rename.

The repair deliberately preserves `canonical.old`'s independent
`{absent, exact_prefix, exact, mismatch}` amendment axis because the exact
retained old target authenticates completion. Marker temporaries independently
retain exact-prefix recovery because their complete 72-byte payload is derived
from the validated intent. Scratch garbage collection is separate,
non-authoritative, and outside HCM-2.2 recovery.

## Mechanical validation before review

The clean documentation worktree reproduced:

- 36 JSON/JSONL parse units with duplicate-member rejection;
- all 13 Draft 2020-12 schemas accepted by `jsonschema==4.26.0`;
- unchanged intent schema: 16,019 raw bytes and
  `sha256:c2f5cf51b833585bc10cfcd5b99ad2a2e2c0e952a79b3f2910ae6afbff746caa`;
- unchanged amendment intent fingerprint:
  `sha256:d7f3c3a9ee860288f0ed52c83254044d67828d2499909633584290d8ec265830`;
- unchanged persisted amendment intent: 7,607 bytes and
  `sha256:ad48fe8c0be1d3ee72888a6409d7e9ef3278623bf2c68c5cb199a3db18a08654`;
- unchanged exact 72-byte marker payload;
- exact ordered `S0`-`S11` boundary closure, all three purpose vectors, and the
  four frozen authoritative stage axes; and
- byte identity between the retained research input copied into this clean
  worktree and its preserved source:
  `sha256:721447c1867d8ed96e169081db9b1e8ec9915f40250ad49f44eebbe3f3fb2915`.

The remaining pre-dispatch wall also passed:

- exact `W0`-`W15`, `R0`-`R9`, and `S0`-`S11` lexical/mechanical closure;
- two YAML contract documents parsed;
- 172 repository-relative Markdown links resolved with zero missing targets;
- normal handoff validation accepted three record schemas, two dispatch
  schemas, two templates, 48 immutable records, 238 current JSON dispatches,
  eight admitted legacy dispatches, and exact parity with 48 ledger entries;
- both historical-v1 admission and orchestration-contract negative self-tests
  passed;
- `git diff --check`, common secret-pattern, and absolute-machine-path scans
  passed;
- all 11 changed/untracked paths are beneath the control pack, with no Rust,
  Cargo, crate, schema-1.2, or other implementation path; and
- no pre-existing handoff, dispatch, review, proof, or released immutable
  record is modified.

The full repaired recovery subject must next be frozen in an exact manifest and
receive fresh isolated read-only review. This proof records no review result
and grants no implementation authority.
