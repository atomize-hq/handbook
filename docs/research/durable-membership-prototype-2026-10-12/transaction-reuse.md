# EchoDB transaction reuse accounting

This bounded experiment implements a transaction shell over **both** existing
adapters. EchoDB contributes interface, lifecycle and test-pattern inspiration;
**zero source lines were copied or adapted**, and `echodb` is absent from the
isolated Cargo manifest/lockfile. The transaction layer and persistence protocol
remain Handbook-owned experimental code. No effort savings were measured.

## Pinned source and attribution

The inspected component is EchoDB 0.8.0 at revision
`17b41ae82f3deba4b7d3034b0c2206a688c29f35`, published under Apache-2.0. The source
headers attribute **Copyright © SurrealDB Ltd**. On 2026-10-12, the official
[published archive](https://crates.io/api/v1/crates/echodb/0.8.0/download) was
retrieved independently; its `.cargo_vcs_info.json` identifies that exact revision.
The pinned [database](https://github.com/surrealdb/echodb/blob/17b41ae82f3deba4b7d3034b0c2206a688c29f35/src/db.rs),
[transaction](https://github.com/surrealdb/echodb/blob/17b41ae82f3deba4b7d3034b0c2206a688c29f35/src/tx.rs),
[errors](https://github.com/surrealdb/echodb/blob/17b41ae82f3deba4b7d3034b0c2206a688c29f35/src/err.rs),
[manifest](https://github.com/surrealdb/echodb/blob/17b41ae82f3deba4b7d3034b0c2206a688c29f35/Cargo.toml),
and [license](https://github.com/surrealdb/echodb/blob/17b41ae82f3deba4b7d3034b0c2206a688c29f35/LICENSE)
were inspected. `lib.rs` and the test-only `kv.rs` were also checked to establish
source accounting. The five Rust files and license exactly match their packaged
counterparts. Cargo's generated normalized `Cargo.toml` differs; packaged
`Cargo.toml.orig` exactly matches the pinned manifest.

| Exact downloaded input | Bytes | SHA-256 |
| --- | ---: | --- |
| `echodb-0.8.0.crate` | 14,063 | `39bf0518aa6636bdbcdbecb62642ef00d4f5505d211b20e686edbe885d5025b7` |
| `src/db.rs` | 6,866 | `29984e7c5191370b9a8e1ce6a8d9149c01960b711d5a038b68d2e34828c4a2bc` |
| `src/tx.rs` | 7,588 | `5d5cdf5d73f3198ebde7f3026ecc8ce69cba34d7f816253795386a279997bed1` |
| `src/err.rs` | 1,050 | `e2ccada423a09ffaf706232c233bd73a971618fd931c4790c274cbecebf7c7cb` |
| Pinned `Cargo.toml` / packaged `.orig` | 732 | `526a38c1427f318f44ec4a48e205f6a64c8417dd239ef1094c662355095496f3` |
| Packaged normalized `Cargo.toml` | 1,447 | `e87e2547b15027010904bd0dd4e29d286742857144549129933f801b65672159` |
| Retained unmodified `LICENSE` | 11,341 | `2165441a9b0e6246a670d9e6219b3f47acd6443afe55a5931288795ecfeb4629` |

[Source provenance](evidence/echodb-source.json) records all seven downloaded
files, packaged comparisons, VCS metadata, exact hashes and measured line counts.
The archive has no separate `NOTICE`/`COPYING` file; copyright/license notices
appear in its source headers. The complete upstream license is retained verbatim
as [echodb-Apache-2.0.txt](licenses/echodb-Apache-2.0.txt), with attribution here and
in `src/transaction.rs`. This documents referenced inspiration; it does not label
new Handbook source as an upstream copy or a maintained EchoDB fork.

## What the pattern contributed

EchoDB starts a read/write transaction from a snapshot, distinguishes writable
and closed states, reads its own changes, checks a point value before a
conditional write, and commits or cancels one transaction. Its inline tests
exercise closed/writable misuse, cancellation, whole-map publication and readers
retaining earlier snapshots. Those specific interface and test ideas informed
`Database::begin`, `Transaction::{get,put,put_if,commit,cancel}` and the new
snapshot/overlay/lifecycle tests. No upstream tests were copied.
[Lifecycle and operations](https://github.com/surrealdb/echodb/blob/17b41ae82f3deba4b7d3034b0c2206a688c29f35/src/tx.rs#L79),
[snapshot tests](https://github.com/surrealdb/echodb/blob/17b41ae82f3deba4b7d3034b0c2206a688c29f35/src/db.rs#L195).

The APIs are related rather than source-compatible: this experiment's `put`
overwrites in the logical overlay, corresponding to upstream `set`; `put_if`
corresponds to upstream `putc`, including absent-value preconditions. Upstream
`put` instead means insert-if-absent. A local read-only commit without replay closes
with `Unchanged`; upstream rejects a read-only commit. A read-only commit with
`replay=true` is refused as `NotPublished`, because terminal replay requires
coordinated publication repair. The CLI applies the `read_only` modifier only
to generic `tx`: request `run`, authority changes and buffered `put` remain
writable semantic operations, so that modifier cannot bypass their publication
or replay boundary. The local shell exposes only
the point operations needed here, and does not implement upstream delete/range
operations. Error strings and publication outcomes are also application-owned.
[Point operations and checks](https://github.com/surrealdb/echodb/blob/17b41ae82f3deba4b7d3034b0c2206a688c29f35/src/tx.rs#L110).

## Why the dependency was not added

EchoDB's implementation is an `imbl::OrdMap` snapshot behind `ArcSwap`, with a
Tokio mutex acquired **before the writable snapshot** and retained until commit,
cancel or drop. Commit swaps the in-memory map. Its four direct dependencies are
`arc-swap`, `imbl`, `thiserror` and Tokio's synchronization feature. The map's own
implementation/maintenance lies in `imbl`, outside the small EchoDB shell.
[Representation and begin](https://github.com/surrealdb/echodb/blob/17b41ae82f3deba4b7d3034b0c2206a688c29f35/src/db.rs#L24),
[commit](https://github.com/surrealdb/echodb/blob/17b41ae82f3deba4b7d3034b0c2206a688c29f35/src/tx.rs#L97),
[dependencies](https://github.com/surrealdb/echodb/blob/17b41ae82f3deba4b7d3034b0c2206a688c29f35/Cargo.toml#L21).

The existing experiment needs two different durable tree adapters, preparation
without holding a process lock, and native publication/recovery. Retaining that
in-memory backend and lock lifecycle would still leave those responsibilities
new, while adding a second map and async synchronization path. Referencing the
small pattern therefore fits this bounded experiment without forcing its
package into the build. This is an experimental design decision, not a claim
that a resident owner model or a different integration was evaluated and lost.

## Maintenance and line accounting

Upstream's five Rust files total **638 physical lines**: 437 outside tests
(278 code, 126 comment-only, 33 blank), plus 184 inline-test lines in `db.rs` and
17 test-only lines in `kv.rs`. There are nine inline Tokio tests. The four-file
implementation count includes headers and exports; it is not dependency tree
size or a count of code reused here.

The following is the final frozen source snapshot recorded with hashes in
`evidence/echodb-source.json`. Code means a nonblank line not beginning with `//`
after whitespace; inline comments remain code. Test columns include their
`#[cfg(test)] mod tests` section. These are physical source counts, not semantic
LOC, executed lines, work estimates or savings. `main.rs` and the adapters
include predecessor implementation; the whole files are not attributed as new
transaction work.

| Current local file | Total physical | Outside-test code | Outside-test comments | Outside-test blank | Test physical |
| --- | ---: | ---: | ---: | ---: | ---: |
| `src/transaction.rs` | 402 | 248 | 16 | 16 | 122 |
| `src/main.rs` | 1021 | 976 | 21 | 24 | 0 |
| `src/chunk.rs` | 534 | 304 | 23 | 18 | 189 |
| `src/disk.rs` | 969 | 570 | 41 | 26 | 332 |

The new transaction module has three unit tests, each exercising both adapters.
The first also checks rejection of read-only terminal replay. The third covers a reused store: native lock loss in a later attempt must report
`NotPublished` despite an earlier successful publication.
Their presence/count is source evidence, not a native platform pass claim;
final tests and process evidence are recorded in the transaction verification
and results documents. A source change requires refreshing this accounting
snapshot before referring to its hashes as final.

Handbook owns the logical `BTreeMap` overlay (256 distinct buffered pairs),
point dependency bound, read-your-writes, conditional checks, closed lifecycle,
whole-root conflict result and explicit revalidation policy. Snapshot reads,
request/item/authority checks, serialization and buffer construction occur
before the native writer lock. Commit holds that lock for native owner/lock/root
checks, exact-root validation and selected point dependency reads, tree changes,
object installation/sync/name boundaries, ROOT publication and required replay
repair. The API returns a conflict and does not replay an arbitrary callback;
the CLI explicitly rebuilds its bounded preparation on a new snapshot where
selected. This is conservative whole-root validation, not read-set OCC,
disjoint-write merging or exactly-once external effects.

Handbook also owns both adapters' bounded batch behavior, process coordination,
content IDs and framing, immutable object reachability/no reclamation, native
rename/flush rules, request/outcome replay, known-prepublication versus possibly
published error reporting, and fault/platform qualification. Each commit attempt
resets its publication marker; a failure returns a typed `CommitFailure` carrying
`Publication::NotPublished` or `Publication::Indeterminate`; the CLI consumes
that typed classification. An earlier successful attempt therefore cannot make
a later prepublication failure appear possibly published. This classifies the
current attempt and does not roll back the earlier commit. The new error wrapper
and its regression are Handbook code, not EchoDB source reuse. Cancel/drop only
discard an unpublished logical buffer; cancel cannot undo possible publication.
EchoDB supplies none of these disk/process guarantees. Platform results, batch
measurements and remaining macOS cases belong to the companion contract and
results, and are not inferred from the referenced upstream interface.
