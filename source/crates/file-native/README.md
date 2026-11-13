# file-native

`file_native` is an opaque-byte Rust database. It owns immutable indexed blobs,
expected-version publication, exact original publication outcomes, retained
reader leases and bounded physical retirement. Domain authorization and record
meaning remain with callers. Handbook supplies its retained-root IO capability;
the default native backend is independently usable without Handbook.

Create and open are separate operations. An interrupted empty creation resumes
only from its positive `CREATING.json` witness and exact binding. `READY.json`
binds the original native gate/lease lock generations. Once creation completes,
the creation witness is removed; missing initialized proof cannot silently
become a new empty database.

A `Transaction` buffers at most 256 changes, with keys up to 1000 bytes, values up
to 1 MiB and a 16 MiB encoded/allocation budget. Original publication keys are
bounded at 1024 bytes. Candidate page/blob identities include fresh OS entropy and a checked per-write
ownership ordinal, so equal values at distinct keys retire independently;
SHA256 covers their exact framed bytes. Index leaves contain at most 64 pairs,
branches at most 32 children, with bounded binary frames and checked height.
Large values are indirect, so routing pages do not embed the dataset. Preparation
reserves no storage. A single native gate serializes publication; expected-root
conflicts become permanently resolvable original outcomes, with no automatic
semantic retry. No graph, projection, process command protocol or permission
token is part of this API.

Before any candidate output write, the required active.json cell atomically
changes from Idle (bound to the exact selector) to Owned (a complete bounded
plan). Both positive and metadata-only negative alternatives, their allocation
union and full control encodings are admitted under unchanged limits first.
Every later initialized control uses reserved staging plus native no-replace
installation. Recovery restores complete displaced predecessors; incomplete
staging is discarded, never adopted. Missing required control is an error.

A transaction may attach one bounded opaque scope/checkpoint with set_evidence.
The complete plan retains its exact bytes before output, bound to the original
input fingerprint, database, predecessor and a fresh positive candidate
token/sequence. Selected roots retain the same binding in original outcome
evidence and one indexed scope head. reconcile_evidence reconciles ownership then
reads that head or an exact older original key by bounded index lookup. Domain
meaning and permission remain with the caller; evidence supplies no authority.

Explicit recovery never installs unselected caller changes. It may select only
the preadmitted negative root, preserving exact predecessor user data while
retaining NotSelected and its separate checkpoint proof. Its value remains empty
and outcome sequence remains the predecessor sequence; the metadata selector
advances. Both positive and negative originals remain indexed and count in
Version.indexed_originals, including conflicts. Updating a scope head deletes no
older original. The native original key cannot be reused. Owned control remains
until proof/event transfer and safe unselected-output cleanup finish, then
atomically changes to Idle. Early pre-Ready creation limits remain; this is no
universal power-loss or arbitrary-corruption recovery claim.

Short current reads may use `with_current`, which holds the validated native
publication gate through a bounded read-only callback and writes no historical
pin. Copy inputs out and leave that scope before publication or maintenance.
The borrowed non-clone view cannot escape; it supplies no domain permission.

Snapshots use eight active native lease locks. Capture and maintenance share the
publication gate. A live old lease conservatively protects all later retirements;
release or process death allows drainage, with no time-based lease expiration.
A bounded history page reports an explicit cursor even through deleted values.
Lease limits are active resource limits, not lifetime transaction limits.

Each selected publication has its own bounded retirement record (at most 4096
object identities), addressed through a fixed-depth numeric hierarchy. One small
checkpoint advances sequence/item coverage. No cumulative Work/catalog/deferred
vector or whole-tree reachability walk is needed. Maintenance bounds each call
by visited items and bytes, persists progress, and stops behind the oldest live
lease rather than skipping a hole. Physical release is reported only after
actual owned file removal. Permanent original outcomes and current records are
retained. The batch/page/allocation limits can refuse before acceptance; history
is not deleted or capped at the demonstrated operating target.

The default Unix backend traverses retained directory descriptors with no-follow
opens and uses native no-replace rename (`renameat2` on Linux, `renameatx_np` on
macOS). Windows retains parent directories without delete sharing and uses
`MoveFileExW` without replacement. File synchronization is required. Directory
synchronization is reported supported on Unix and unsupported on Windows; the
latter is not a uniform power-loss durability promise. Native macOS and Windows
validation is a separate delivery obligation. Linux tests prove only their
named process-interruption/native scope, not arbitrary corruption or power loss.

`cargo test -p file-native --features test-support --test public_api` exercises
valid-state publication, exact originals, expected-version contention, native
no-replace sharing, creation/publication interruption, old reader retention,
process-death lease release and bounded history/retirement continuation. Fault
hooks exist only with the non-default test-support feature. No research CLI or
malformed/corruption continuation is invoked by these tests.
