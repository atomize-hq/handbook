# History tail method options and failed-attempt admission

Draft for coordinator method review; **no workload change or execution grant**.
The original mixed H1002/H10002 cohorts and their frozen optimized-lookup cutoffs
remain unchanged. Current F05 admission remains 1 GiB; the prospective 768 MiB
reduction has not been applied. No planned physical collection credit is used.

Source inspected: exact provisional CP3
`f9b762a1a0d352564542bb4850ad77ee14a7c6e4`; its preparation/selection/lifetime
semantics must be rechecked or proved equivalent at the final source pin.
GitNexus query could not resolve this unregistered checkout; the failed query is
retained at `C:/file-native-online-matrix-20261020/receipts/history-attempt-bound-query.txt`.
All findings below come from the exact committed Rust blobs, not a mutable binary.

## Existing method and proposed difference

The existing baseline history uses the authentic mixed resident history recipe,
including content/metadata/no-op requests, in bounded chunks. The continuation
requires the same operations and load between its no-maintenance controls and
online cycles. The checklist permits explicitly labeled no-op history, but that
does not silently approve changing an already frozen continuation recipe.

An alternative would predeclare **every new tail request** in both control and
maintenance phases as `ops=[]`, `packet=false`, explicit captured base, with a
unique ASCII request ID of at most 64 bytes. The initial mixed history would be
preserved, and optimized lookup would still run first at H1002/H10002. All 1,600
new accepted publications, membership entries, outcome entries, attempts and
actual H intervals would still be measured. The current N128 graph and authored
sources would stay fixed during this tail; the separate F07 payload/title pairs
would remain the mutation workload. This narrows the tail's interpretation to
publication/history maintenance under contention; it would not measure mixed
payload/metadata transition costs under that load.

No alternative is implemented here. Keep the mixed continuation until the
coordinator reviews and records an explicit method decision.

## Two distinct semantic conflicts

`Store::prepare` (`store.rs:201-221`) checks an explicit captured base at line209,
before creating an attempt or reserving space. `conflict:captured-base` creates
no new candidate or reservation for that call. Existing transfer recovery may
update the shared registry; it is not new private attempt allocation.

`Store::select_locked` (`store.rs:338-345`) checks `c.expected` after preparation
and sealing but **before** selection grant, membership/outcome COW writes, result,
publication, allocation record, hint or selector stage. Thus an authentic
`conflict:expected-publication` leaves a sealed reservation and candidate. This
distinction must come from the raw exact input, expected error, binary binding
and owner evidence, not from treating every failed call as a harmless conflict.
Any uncertain/lost reply must be resolved as acceptance before abandonment.

## Conditional no-op byte bound

These bounds apply only after checking the following preconditions on the
actual final-pin H stores: mode A; N128/project; graph fallback false; projection
profile4; packet-not-requested derived files already present with exact hashes;
initial reservation registry empty and no other semantic writer or owner;
request `ops=[]`, `packet=false`; request ID ASCII and <=64 bytes; store ID ASCII
and <=77 bytes; captured publication ID ASCII and <=99 bytes. They do not apply
to mixed-operation tails, B generations, packets, projection upgrades or damage.

With empty operations, graph transition creates no changed documents or COW
updates (`graph.rs:121-128,174-175,225-242`). Empty index updates preserve their
root without writing (`objects.rs:48-52`, `disk.rs:592-594`). Matching profile4
projection produces no navigation delta (`navigation.rs:94-96,131`) or changed
paths. The two static packet-not-requested derived files are checked in place;
their paths/hashes are still recorded in `replay_files`
(`projection.rs:24-32,50-58,117-146`). Therefore replay_objects is empty,
replay_files has exactly two 75-byte paths and 64-byte SHA values, and graph,
projection and changes roots retain their identities before selection.

`io::unique` (`io.rs:82`) has prefix + three separators + PID(u32, <=10 digits)
+ nanos(u128, <=39 digits) + counter(u64, <=20 digits). Consequently attempt IDs
are <=79 bytes and generated store IDs <=77 bytes. The bound on an existing
captured publication uses its stricter safe-ID contract, <=99 bytes.

Using the closed Candidate field order (`store.rs:24-30`), closed Request and
GraphRoot fields, all maximum widths above, all u64 graph scalars at 20 digits
(count fixed128), SHA strings at64 bytes, empty changed/removed/replay arrays,
null generation/checkpoint/packet fields, and Rust's two-space pretty JSON with
no final newline gives **1,784 bytes per sealed no-op candidate**. This is a
field-width bound, not a uniform per-publication allowance. Preparation's quota
counter is reset after reserve; candidate write charges body+4096, and sealing
records used+65536 (`store.rs:216-220`, `io.rs:141-145`, `lifetime.rs:30-34`):
sealed used <= **71,416**. The reservation allowance remains exactly64 MiB.

A maximum abandoned reservation object is535 bytes standing alone. A registry
of eight such entries with79-byte distinct keys is **5,154 bytes** including
its actual nested indentation. Atomic replacement briefly retains destination
plus one complete registry stage: **10,308 bytes**. Registry mutations are under
the semantic writer lock. Abandonment adds the six-byte `fenced` marker, whose
own atomic stage is another six bytes while being installed.

The resulting bound for **2,400 extra sealed losers**, with every candidate and
abandonment marker retained and no deletion credit, is **4,296,000 logical
bytes and4,800 files**. Add the shared registry/stage high-water10,308 bytes,
one marker stage6 bytes, and conservative four in-flight candidate bodies7,136
bytes: **4,313,450 bytes**, plus the separately admitted accepted-publication,
physical-coexistence, inventories, logging, directory and other temporary costs.
The added persistent directories are <=2,400 attempt directories; directory
allocation and device allocation are not inferred from logical file bytes.
For a gross allowance of4,800 sealed losers instead, double the retained
candidate/marker portion; that broader bound is not silently replaced by2,400.

The2,400 count assumes eight four-publisher phases at <=400 offers/100 accepted,
with the eight one-publisher phases having no external semantic writer and no
unexpected semantic conflict retries. Unexpected single-publisher conflicts
must fail qualification or receive a newly admitted bound; they are not hidden.
All actual offers/conflicts/successes and logical-slot end-to-end times remain
separate from successful-call engine times. Four frontends must actually overlap.

## Reservation capacity remains a blocker

The smaller file bound does **not** solve live quota admission. `lifetime.rs`
limits the registry to eight entries (lines6,22-28). A successful publication
transfers its own reservation; failed sealed preparations remain charged.
`lifecycle abandon` (lines99-107) requires exclusive preparation access, refuses
the selected attempt, installs a durable fence, and marks the entry abandoned.
It returns `abandoned-charged-until-gc`; it does not release an entry or quota.

The existing release of abandoned reservations occurs only in semantic
`lifecycle collect` at lines233-258, after actual owned-file removal. That
collector enumerates accepted membership/evidence and retained content roots
(lines137-151), and runs under writer/exclusive-preparation locks. It is not a
bounded point cleanup operation. Calling it repeatedly during the timed load
would introduce additional history-dependent work and must be explicitly
reviewed/accounted in both variants, with raw calls, waits and actual release
censuses retained. It cannot be hidden as physical incremental maintenance.

Before any tail execution, the coordinator must resolve both issues: approve
the workload method and admit all failed-attempt bytes/files/temporaries, and
define a legitimate quota-release schedule or source-owner resolution that
preserves four-front-end overlap and all offered-load counts. Abandonment alone
does not establish completion under the admitted worst-case conflict pattern.
A natural run might have few post-preparation losers, but that is not a bound
that guarantees reservation capacity throughout the phase. No engine change,
collection, retirement, deletion, quota credit or real tail execution occurred
while preparing this document.
