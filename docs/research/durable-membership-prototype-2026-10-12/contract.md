# Experimental contract, 2026-10-12

The bounded transaction follow-up extends this contract as specified in
[transaction-contract.md](transaction-contract.md). Final-code comparison and
evidence are in [transaction-comparison.md](transaction-comparison.md); predecessor
measurements retain their original version and do not establish the new interface.

Authorized two-way experiment only. No production format, capacity, owner, backend,
dependency, or migration selection. The structured-data standard remains proposed;
existing Contracts JSON and owner/version contracts are unchanged.

## Executable obligations

1. One root binds request membership, exact original request fields/proof/outcome,
   item reservation, modeled authority revision/scope, and pending/attempt/terminal
   status. Membership is independent of optional payload files.
2. Exact committed retries replay the retained outcome even after revocation.
   Changed content on a retained request is refused. Different requests cannot
   both reserve one item. Pending/resumed effects require current original proof.
3. Prepare outside a cooperative OS process lock. Compare the expected root under
   that lock; on conflict reopen and revalidate request/item/authority reads. No
   acknowledged update may be lost. Locks cover publication, never agent work.
4. Reserve, accept, possible effect attempt, terminal commit, and reply are distinct.
   Attempted with no terminal record is uncertainty; recovery cannot infer noncommit.
   No arbitrary external effect or exactly-once effect claim is made.
5. Immutable objects are framed, length bounded, checksum addressed, decoded with
   closed schemas, and structurally validated. Missing/corrupt selected objects
   fail explicitly. Recovery follows the selected root, never the newest survivor.
6. Write nodes/payload prerequisites, sync them and their names, stage/sync a root,
   publish an expected-root transition, establish the selected platform publication
   durability boundary, then acknowledge. Faults after replacement are ambiguous.
7. Native path checks reject symlinks/reparse points at the experiment store boundary;
   retained directory/lock identity is checked across publication. Same-user malicious
   concurrent directory replacement/noncooperating writers are excluded; hashes prove
   integrity, not permission/freshness. This is not the Handbook native guard.
8. Exact path checks and an explicit broad audit are compared with a separately
   instrumented sibling enumeration guard. This does not select production damage
   detection semantics. All objects/proposals/old roots are retained, so held readers
   stay reachable. No GC claim; storage amplification is measured.

## Formats and bounds

Harness messages and evidence use JSON. Experiment nodes use compact closed JSON
inside a checksum envelope, at most 1 MiB per object (follow-up; predecessor 128 KiB), at most 64 pairs per leaf/
chunk, disk internal fanout 32, depth at most 64, eager decode node budget 1,000,000.
Durable IDs are SHA-256 of exact object bytes; live allocation addresses are only
ephemeral codec memoization keys and must be pinned while memoized.
Keys and values share the same 1,024-byte domains between variants. CLI input is
bounded to 4 MiB to admit 256 maximal escaped pairs. Request strings are exact
UTF-8 bytes with explicit bounded length. Original equality covers the decoded
request/item/content/proof tuple, not raw CLI JSON spelling or preparation controls.
Authority is a scoped epoch/proof model,
not the full Handbook grant/evidence calculus.

## Evidence boundary

Required: Windows/NTFS and Linux kernel/ext4 runtime tests, subprocess concurrency,
fault/kill/reopen checks, growth at 1k/10k/100k unless a measured resource bound stops
it, repeated raw samples, instrumentation overhead. Fresh process is not cold OS
cache. Native macOS and hardware power cuts are separate qualification gaps unless
actually run. No local witness can detect coherent all-local rollback.
