# P physical representation hypothesis

The fixed research rules and scope are those in the reconciled October 19 brief,
with the human's 40 GiB amendment. This is a standalone private experiment. The
structured-format standard remains proposed; existing exact source and semantic
owner contracts control. New catalog/journal records use closed, duplicate-free
JSON because operations own them. Existing record bytes are never reserialized.

Store version 1 means loose storage. Version 2 requires a SHA-256-bound immutable
physical catalog in STORE.json; old binaries refuse both its version and field.
Initial activation requires quiescence with old processes closed. A shared
physical read lease is obtained before STORE/catalog capture and retained in the
operation's Metrics for the entire operation. New Metrics created internally
capture their own lease before resolving an eligible record. Registration and
deletion take the exclusive physical lease; cancellation drops it through RAII.
The existing preparation/lifecycle lease protects the build cohort from retirement
while allowing semantic publications. Registration checks expected physical
configuration, and new publications remain loose. Semantic expected-head checks
are unchanged.

Typed keys are exact role-specific relative paths: selected publication/result/
accepted hint, then finalized accepted-attempt candidate/allocation records after
transfer reconciliation. Reservations and active work, packet files, controls,
sources, views, generations and COW pages stay loose. Packing eligibility is
8 MiB and does not change supported input limits. Content retirement continues
to refuse historical content even while bytes remain in a partly live pack.
Explicit repack drops retired attempt records only after semantic protections
and packet-owner reconciliation; publication/results/hints remain permanent.

Pack bytes concatenate independent raw records or level-3 Zstandard frames.
Grouping uses sorted typed keys and raw-byte/record/index limits, identically
for P0 and PZ. There are no deltas, dictionaries, codec threads or mmap.
The binary index has magic/version, pack SHA-256, actual pack length, codec,
count, and sorted entries containing key, checked offset, encoded length, raw
length and raw SHA-256. It binds complete contiguous coverage of the pack.
The catalog binds index digests/sizes, pack digests/sizes, role and key ranges.
Index bytes plus routing offsets have a 64 MiB cache budget; payload reads are
bounded separately. Mapped missing/corrupt records refuse; unmapped keys alone
use loose storage. Exact staging duplicates are allowed and conflicting ones
refuse before registration/deletion. The catalog cannot establish membership.

Maintenance reserves before construction, retains durable owner/journal records,
verifies and installs immutable pack/index/catalog before atomic activation,
then separately deletes loose/old pack copies after readers drain. Only successful
physical deletion credits reclaimed bytes. Windows sharing refusals remain
charged and retryable. Interrupted construction remains owned; resume never
infers successful deletion from unreachability. Original selected allocation
owners are never rewritten.

GitNexus is UNKNOWN: the exact new worktree is unregistered and the available
shared graph predates this engine. Explicit cached impact attempts are retained.
Manual tracing: io read_bounded -> checked -> Store publication/result/candidate
and retry hint; Store publication -> member/accepted/snapshot/checkpoint,
projection refresh/protection and lifetime collection; API run -> all consumers.
These are broad experimental storage paths and need the full scoped fault and
independent byte/semantic/operational restore checks, not a clean-graph claim.
