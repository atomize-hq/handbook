# HCM-2.2 Authority-Repair Review 4 Remediation Proof

## Review result

Different fresh isolated reviewer `/root/hcm_2_2_authority_repair_review_4`
admitted all 17 paths and aggregate
`sha256:bb340bf6227a16dae18259625511eb7a39313f1a6606fa938564bbbb970ba67e`
from immutable dispatch
[`../../../handoffs/dispatches/20260720T150823Z--HCM-2-2--authority-repair-planning-review-4.json`](../../../handoffs/dispatches/20260720T150823Z--HCM-2-2--authority-repair-planning-review-4.json).
It returned `CHANGES_REQUIRED` with one Required finding,
`HCM-2.2-ARPR4-001`, and no additional finding. The finding was accepted
without waiver.

## Finding and repair

Committed terminalization was not symmetric with the crash-closed rollback
terminal. A crash after `<id>.pending` was renamed to `<id>.committed` but
before the transaction-parent fsync was observationally identical to an exact
terminal, while the terminal row could return without replaying that fsync.
Destination collision and unsafe-destination behavior was also implicit.

The repaired contract now freezes the exact forward terminal boundary:

1. `W14` is explicitly a matching published `committed` marker under
   `.pending`, with exact finals and no staged output;
2. recovery reverifies the exact pending terminal payload;
3. rename-no-replace is the only permitted move to `<id>.committed`;
4. any exact, mismatching, unsafe, or otherwise pre-existing destination,
   simultaneous suffix, or byte/name mismatch preserves the complete pending
   journal and refuses without mutation;
5. after a successful rename, recovery fsyncs the transaction parent and
   reverifies the exact terminal set; and
6. every observation of an exact `.committed` terminal repeats the parent fsync
   before returning because prior completion cannot be observed after a crash.

The SPEC recovery table, base contract, intent vector, implementation plan, and
checklist now freeze the same rule. The vector adds the exact terminal-
destination axis and a collision negative. No prior dispatch or proof was
rewritten.

## Remediation validation

- the unchanged intent preimage/fingerprint, 7,607-byte persisted document,
  document SHA-256, and 72-byte marker must reproduce exactly;
- both schemas must meta-validate and the positive intent must validate;
- conformance enumerates committed-marker-published pending state across
  `{absent, exact, mismatch, unsafe}` destination states, plus simultaneous
  suffixes, terminal rename, parent-fsync crash, and exact terminal replay;
- only absent destination selects rename; every collision/non-writer state
  selects one evidence-preserving mismatch row;
- fault injection immediately before/after committed terminal rename and
  parent fsync must retry to one exact immutable terminal;
- rollback `R0`-`R9`, all identity/currentness/migration/status/path/API
  boundaries, local links, whitespace, and documentation-only scope must remain
  unchanged and passing.

The next review must use another different fresh isolated read-only reviewer
over a new exact complete-subject manifest. This proof grants no implementation
or HCM-2.3 authority.
