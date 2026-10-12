# Bounded transaction follow-up

Non-production experiment; stop at the Windows/Linux comparison for human review.
No production backend, migration, format or capacity is selected. The structured
format standard remains proposed. Existing owner/version contracts are untouched.

Question: does an EchoDB-inspired application transaction over each existing tree
allow real snapshot reads and buffered logical preparation outside the process
lock, preserve the modeled semantic transition, and remove disk intermediate batch
paths at a useful maintenance cost?

`Database::begin` captures the positively established native owner/lock and selected
root. `Transaction` offers point `get`, bounded buffered `put`, `put_if`, `commit`,
and `cancel`; drop discards its overlay. Reads see the overlay first. Failed
commit returns typed `CommitFailure` with per-attempt `NotPublished` or
`Indeterminate`; a reused object store cannot inherit the prior commit's error
classification. A read-only
transaction retains its snapshot across other process commits under no reclamation.
Closed transactions refuse reuse. Values are bytes; CLI writes remain unrelated
`u:` keys. This is application-owned code inspired by EchoDB 0.8.0, not an EchoDB
backend/dependency, copied implementation, generic query engine or read-set MVCC.

Commit closes the transaction, takes the native writer lock, rechecks native
owner/lock and exact expected root, re-reads recorded point dependencies, applies
the overlay to a private tree proposal, writes/syncs prerequisites, and publishes
one selected root before acknowledgement. Whole-root mismatch returns an explicit
conflict without applying the overlay. The generic transaction never retries a
callback. The CLI's `run`/`put`/`authority` wrapper may explicitly re-begin and
revalidate its pure command, outside the lock; `retry:false` returns conflict.
`tx` always returns conflict for caller-controlled revalidation. Arbitrary agent
work and external effects are never retried by this interface.

The command's request/item/authority reads, original/stage checks, serialization,
overlay construction and conditional checks occur before locking. Only the test
overlap control sleeps; that pause is not logical work. Tree mutation, dependency
recheck, object installation/flush and root publication remain inside the lock.
Success, known conflict/prepublication failure and indeterminate possibly-published
failure are separate. Cancel cannot undo possible publication. Exact semantic
terminal replay re-establishes publication durability before reply, including after
revocation. `read_only` only selects the generic `tx` mode; semantic `run` replay
always takes coordination and repairs durability. A read-only application tx
explicitly refuses a replay-repair commit. Attempt without terminal remains
uncertainty, not exactly-once effects.

The same 1,024-byte UTF-8 key / 1,024-byte value and 64-pair node capacity remain.
Both object decoders/encoders admit up to 1 MiB (was 128 KiB). A 64-pair chunk with
maximal JSON key escaping (6 bytes per byte), byte-array values (4 per byte), framing
and child metadata fits below 657 KiB; a 32-child disk branch fits below 400 KiB.
The shared CLI limit is 4 MiB for 256 maximal escaped input pairs. Wire/node spelling
and content addressing stay unchanged. This experimental bound change requires
fresh/versioned comparison evidence; predecessor measurements remain historical.

Disk `put_batch` groups updates through each existing path and emits final affected
nodes once. Chunk buffers all COW updates before final encoding. `coalesce:false`
uses sequential adapter puts and persists each intermediate tree, under the same
single root publication, for a comparable final-code baseline. No objects are
installed outside coordination and no GC, cache or lazy chunkmap rewrite is added.
