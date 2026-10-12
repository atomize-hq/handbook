# Immutable-chunkmap reuse accounting

This experiment uses the unmodified registry package `immutable-chunkmap =2.1.4`,
with `default-features = false` and 64 pairs per chunk. It does not select a
production dependency or fork. Upstream default features are already empty; the
explicit setting also makes the bounded experimental configuration reproducible.
Pooling, Rayon and upstream map Serde are unused.

The published archive was retrieved independently on 2026-10-12. Its
`.cargo_vcs_info.json` identifies revision
`e600c70f70a79df05fb38dc4a74aa919dd399d2b`, matching the research review and the
source used to implement this adapter. The downloaded `avl.rs` and `tests.rs`
exactly matched their packaged counterparts by SHA-256. Reproduction uses the
isolated Cargo manifest and lockfile; the temporary inspection directory is not
an authoritative input.

| Input | SHA-256 |
| --- | --- |
| Published `immutable-chunkmap-2.1.4.crate` | `da5fe4d67d09ce2c08ff406f6758ec716206c28be92357ae5d5f798c2ef4b548` |
| Pinned and packaged `src/avl.rs` | `7628a7e9398a4ad6fd6850b046d5a676c9e300f5bdf8fc94eed1cc80da9f698b` |
| Pinned and packaged `src/tests.rs` | `36139e44cc684e5bf151fa06fc3d84472a648e54281437c2c5cea19ddb25a1b0` |
| Retained `LICENSE-MIT` | `267f4a928bb3cf6ad37252621eb1199d2cb58560046b7dea4ec3f7a4cc9261a7` |
| Retained `LICENSE-APACHE` | `7277e044486d82039c8855cf1611f57e64b7980213ba3589db49daf09dbdfd3e` |

Primary sources:
[published package](https://crates.io/api/v1/crates/immutable-chunkmap/2.1.4/download),
[pinned manifest](https://github.com/estokes/immutable-chunkmap/blob/e600c70f70a79df05fb38dc4a74aa919dd399d2b/Cargo.toml),
[structural APIs](https://github.com/estokes/immutable-chunkmap/blob/e600c70f70a79df05fb38dc4a74aa919dd399d2b/src/avl.rs#L2066),
[sharing codec test](https://github.com/estokes/immutable-chunkmap/blob/e600c70f70a79df05fb38dc4a74aa919dd399d2b/src/tests.rs#L1222).

Upstream declares `Apache-2.0 OR MIT`, and the MIT notice names Copyright 2022
Eric Stokes. The complete unmodified notices are retained in `licenses/`.
The required direct tree dependency is `arrayvec` (resolved version is in
Cargo.lock). No optional upstream dependencies or source file copies are needed.

The reused implementation supplies ordered-map lookup, bulk construction,
snapshots, path copy-on-write, chunk splitting and relaxed-AVL balancing. The
adapter uses `Map::root`, `Map::from_root`, `NodeRef::{identity,keep,pairs,left,right}`,
`NodeHandle::{create,view}`, `insert_cow`, `get`, `len`, and `FromIterator`.
The source review counted 4,329 physical upstream implementation lines and 1,369
dedicated test lines. Those describe the dependency maintenance surface, not
lines executed by this prototype or engineering effort saved.

No upstream tree code was rewritten. The initial formatted adapter is 534
physical lines: 345 implementation/comment/blank lines and 189 test lines.
These counts are source accounting, not measured engineering effort. Handbook
owns the new `src/chunk.rs` adapter, its closed JSON node schema, validation,
resource limits, stable object
addressing, codec memo, and focused model/malformed-input tests. Its codec pattern
is informed by the pinned sharing-preserving upstream test; its durable format
and validator are new. Handbook also owns all common process coordination,
publication, durability, semantic request handling, fault injection and recovery
in this experiment. None of those are supplied by the tree dependency.

Reopening eagerly reads and reconstructs **every** selected-root node before
lookup; warm `get` performs no object reads. The constructor is unsafe, so all
descendants, strict key ordering, complete subtree bounds, pair counts, height,
size and relaxed balance (height difference at most **two**) are validated before
each call. JSON objects reject unknown and repeated fields, use at most 128 KiB
per node, have at most 64 pairs, and obey depth 64 / eager-node budget 1,000,000.
Keys and values are bounded separately. These are experimental contracts.

The encoder memo stores a stable checksum ID alongside a **pinned** NodeHandle.
A live allocation address is only an ephemeral memo key and never appears in
persisted bytes. A memo hit stops encoding traversal at a shared subtree, so a
small mutation writes newly changed nodes instead of a full-map Serde snapshot.
Snapshots share the memo through an Arc and clone in O(1). For simplicity, encoded
historical nodes remain pinned for the lifetime of the live snapshot lineage;
reopening starts a memo with only currently reachable nodes. This additional
resident-history memory growth is a real limitation of this adapter, not an
inherent claim about the upstream tree. A pruned memo or persistent owner service
would need separate lifetime accounting and measured startup/memory costs.

Object reads/writes and their byte counts are measured by the common store.
Newly persisted objects are a write-work metric; they do not count all allocation
or COW-copy events inside the unmodified dependency. No allocator or upstream
node-copy instrumentation is added, and unavailable measurements must be labeled.

## Focused verification

On the native Windows host, `cargo test --manifest-path
docs/research/durable-membership-prototype-2026-10-12/Cargo.toml chunk::tests --
--test-threads=1` passed all four module tests on 2026-10-12. They cover deterministic
randomized model equality, reopen/mutation, snapshot isolation, retained old
roots, incremental reuse, invalid schema/IDs, repeated/unknown fields, trailing
documents, unordered/duplicate/overfull pairs, missing nodes, wrong child bounds,
height/size disagreement, relaxed balance, and input rejection before mutation.
The full experiment's Linux/publication/concurrency/fault results are recorded
separately; this module-only test result does not establish them.
