# Work-state capacity and performance investigation

Date: 2026-10-10. Status: **investigation complete; design recommendation PROPOSED**.
Baseline: `5363e0a45028ace83363b96b628d41395689441e` on
`feat/handbook-contract-membrane-clean`. This report supplies research for human
selection. It authorizes neither a storage policy nor implementation planning.
Production source, configuration, limits, fixtures and retained user stores are
unchanged. The unrelated untracked research present at admission was preserved.

The measured 32-slot store still supports exact/current reads, but warm current
item medians are about 1.3–1.65 seconds, and fresh revocation refuses at saturation.
At 31 slots, observed fresh registration effects take 7.27–8.74 seconds, before
adding separately measured preparation. The conditional recommendation is an
Engine-owned segmented history with authoritative indexes, exact proof reuse and
protected administration/closure capacity. Its integrity and recovery proof,
workload targets and actual retained-store treatment require human selection.

## Question and controlling boundaries

The accepted compositions/kinds slice provides useful exact registration and
recovery, but its shared 32-transaction lifetime budget cannot support sustained
project use. An initial grant and one kind leave at most 30 item registrations;
accepted abandoned, pending and noncommitted transactions also consume slots.
Fresh revocation uses the same admission budget. These are implementation
constraints, not intended project-size semantics.

Authority comes from the [selected follow-up](../specs/handbook-contract-membrane/coverage-and-landing-map.md#work-state-capacity-and-performance-follow-up),
[completed slice](../specs/handbook-contract-membrane/slices/work-compositions-and-kinds/README.md),
and its accepted [Packet 1](../specs/handbook-contract-membrane/slices/work-compositions-and-kinds/packet-1.md)
and [Packet 2](../specs/handbook-contract-membrane/slices/work-compositions-and-kinds/packet-2.md).
The [architecture](../specs/handbook-contract-membrane/architecture-context.md)
keeps semantics inward of SDK. The [format standard](../specs/handbook-contract-membrane/structured-data-format-standard.md)
has **PROPOSED general defaults**, **Contracts JSON fixed**, and **existing
owner/versioned contracts controlling**. This research adopts no general default,
database, format migration, permanent old API, history deletion or new execution unit.

Immutable semantic invariants:

- Preserve exact submitted and effective meaning, canonical identity domains,
  original receipts, per-version kind approval, and all required historical dependencies.
- One current registration lineage; human-only scope successors, unchanged-scope
  D→G, pending administration suspension, revocation and no reinstatement. A kind
  establishes meaning, supplies no registration permission, and survives grant revocation.
- Hold current native authority through the effect. Original committed recovery
  precedes current permission; pending recovery uses the original request and
  supported original authority rules. A fresh key or successor basis cannot repair uncertainty.
- Positive acceptance and terminal/noncommit proof must survive crashes. Missing
  declared effects are uncertainty, never absence or permission. No latest-file selection.
- Retain the exact physical repository/owner/lock identity and coherent serialization
  of competing effects; a replacement index or database is not independently authoritative.

Changeable mechanisms include slot count/layout, proof duplication, indexes,
validation algorithms, segments, transaction storage and per-operation budgets.
The semantics do not themselves require the present full-history scan algorithm.

## Proposed workload envelope

These ranges are **assumptions for selection**, not observed customer workloads,
defaults or product ceilings. Counts are lifetime, not simultaneously open items.
An accepted request adds history even when it does not ultimately commit. Calls
replaying one original locator are reads of that history, not additional transactions.

| Scenario | Committed items / lifetime | Committed kind versions | Committed work-authority administrations, including initial grant | Additional accepted abandoned/pending/noncommit requests | Peak active-day mutations / concurrency |
| --- | ---: | ---: | ---: | ---: | --- |
| Small, weeks to months | 100–500 | 1–10 | 2–20 | 0–50 | 10–100 mutations/day; 1–2 callers |
| Medium, months to years | 1,000–10,000 | 10–100 | 20–200 | 10–1,000 | 100–1,000/day; 2–8 callers, short bursts |
| Large, years | 10,000–100,000 | 100–1,000 | 200–2,000 | 100–10,000 | 1,000–10,000/day; 4–16 callers |
| Long-lived extension | 100,000–1,000,000 | 1,000–10,000 | 2,000–20,000 | 1,000–100,000 | Accumulation across releases; active population may stay small |

The daily rates describe busy-day bursts, not steady annual arrival rates. A
long calendar lifetime can include substantial inactive time. These assumptions
must be replaced or revised when actual adopter workloads become available.
Transactions are approximately the sum of the four count columns for delivered
operations; the additional-request column excludes requests already counted as
committed items, kinds or administrations.
Even the small range exceeds today's capacity. Future relation revisions,
eligibility records and closing attempts may add several records per item; the
multiplier depends on separately selected features and is not designed here.

Byte planning must include original authority proof, decisions, snapshots,
terminals, slot metadata and temporary publication, not just the item label.
Measured retained sizes and their limits appear below. For design sensitivity,
use **16–256 KiB per transaction after shared-proof storage**, plus a separate
exact authority-proof population. This is an estimate, not demonstrated compression
or a record-size ceiling; the measured kind locator is itself about 33 KiB.
It gives roughly 16–250 MiB for 1,000 transactions, 1.5–25 GiB for 100,000, and
15–245 GiB for one million, before indexes, filesystem overhead and temporary
closure reserve. Retaining today's duplicated proofs requires its measured
bytes/transaction instead. No finite disk can retain an unbounded history.

## Reproducible measurements

The [research directory](work-state-capacity-and-performance-2026-10-10/) contains
the standalone harness, dependency lock, compact raw samples, summary script and
environment metadata. It uses the current Engine/SDK path dependencies and supplied
public-owner setup helpers; it does not patch production symbols or fabricate owner
state. All stores are fresh task-owned temporary roots, populated and reopened in
place without copying or resetting physical identity.

Platform: Windows 11 Home x64 `10.0.26200.7462`; i9-12900KF (16 cores/24 logical
processors), approximately 128 GiB RAM; local C: NTFS NVMe SSD. Rust/Cargo 1.89.0,
MSVC target, LLVM 20.1.7; Cargo release defaults (`opt-level=3`, no LTO), two build
jobs, 32 MiB worker stack. SDK default features, **Engine `test-support` compiled**
for real-boundary recovery and observations. This is not a separately measured
default-Engine-feature binary. Product dependency name/version/source triples
match the workspace lock. The earlier unlocked pilot is excluded from final data.

Timers use `std::time::Instant` inside the process. Setup, population, recursive
byte counting, and original locator cloning occur outside effect timers;
preparation and effects are measured separately. Input label is 4 UTF-8 bytes,
work question 27 bytes, ordinary leaf or its exact kind equivalent, fixed supplied
small native-authority cohort. Repeated single-caller read/component groups have
one untimed warmup then five observations; contention uses three pairs without a
dedicated paired warmup. Exact-size mutations/preparations have one observation without an
identical-effect warmup. Native effects at 16/31 include lightweight boundary
timestamp observations, labeled separately from ordinary SDK effects. Repeated
reads use warm process/filesystem caches; no dedicated idle machine, cold cache,
large authority cohort or power-loss timing is claimed.
Kind establishment, governance establishment and independent local-check
revocation are untimed setup. Standalone kind-publication latency is not measured.
The baseline shares one native authority basis, enabling same-proof validation
reuse within each scan. The scope cohort changes work-grant lineage on that same
native basis; it does not measure many distinct captured native authority histories.

Reported bytes are regular-file contents beneath `.handbook/work-state`, not
allocated disk blocks or the exact 128 MiB owner accounting; root anchor/bootstrap
are excluded. Capacity refusals are separate from successful operations. Tables
use individual mutable observations and read median/range with sample counts;
**no p95 confidence is inferred from five samples**.

### Retained-history growth

All latencies below are **milliseconds**. Current-read cells are medians of five
warm calls. Each prepare/effect cell is one observation; `†` marks an instrumented
native effect, while other effects use SDK. Native preparation receives an already
observed basis; SDK preparation includes its basis observation. At 32, the effect
is a verified `Refused/work_capacity`, not a successful registration. The 2-slot
kind store has no item to read yet.

| Input | Slots at entry | SDK current read median | Native prepare | SDK prepare | New registration effect | Retained bytes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| explicit | 2 | 187.549 | 752.743 | 913.330 | 2207.969 | 508,114 |
| explicit | 3 | 216.847 | 812.165 | 938.083 | 2256.440 | 753,663 |
| explicit | 8 | 399.142 | 1150.073 | 1263.008 | 3028.338 | 1,981,408 |
| explicit | 16 | 651.650 | 1674.509 | 1858.629 | 4370.519† | 3,945,860 |
| explicit | 31 | 1299.821 | 2997.310 | 3077.073 | 7266.455† | 7,629,245 |
| explicit | 32 | 1323.122 | 3022.024 | 3256.397 | 3000.633 refused | 7,874,804 |
| kind | 2 | — | 753.803 | 894.614 | 2168.339 | 506,664 |
| kind | 3 | 229.430 | 853.483 | 969.267 | 2369.391 | 804,791 |
| kind | 8 | 459.626 | 1281.240 | 1455.706 | 3351.934 | 2,295,426 |
| kind | 16 | 802.219 | 1999.215 | 2107.985 | 5012.528† | 4,680,502 |
| kind | 31 | 1627.332 | 3623.332 | 3691.708 | 8737.106† | 9,152,557 |
| kind | 32 | 1641.926 | 3699.330 | 3961.182 | 3704.658 refused | 9,450,694 |

In this mix, each additional explicit item retained approximately 245,550 bytes;
each kind-backed item approximately 298,130 bytes. The 4-byte label is not the
storage cost. These small fixed-authority figures are not general maximum item
sizes. Current-read cost grows with retained history even when selecting the same
first item. At 31 slots, kind-backed SDK current reads ranged from 1595.892 to
2174.046 ms around a 1627.332 ms median: retain that slower observation, rather
than present the median as a latency guarantee.

### Exact and current reads at full ordinary capacity

Every cell is a median and observed range in ms, **n=5**. Exact original and
committed recovery assert `Replayed`; current item asserts `Open`; current terms
assert the supported current grant. SDK current-terms resolution obtains a fresh
native basis. These successful read/replay results remain distinct from fresh
capacity refusal.

| Input / operation | Native median (range) | SDK median (range) |
| --- | ---: | ---: |
| explicit: Exact original | 1320.400 (1308.755–1323.894) | 1347.673 (1322.236–1351.257) |
| explicit: Current item | 1318.265 (1314.234–1426.727) | 1323.122 (1316.825–1372.853) |
| explicit: Committed recovery | 1337.976 (1332.135–1352.585) | 1327.928 (1311.672–1337.185) |
| explicit: Current terms | 1701.774 (1695.952–1726.367) | 1835.715 (1819.402–1857.499) |
| kind: Exact original | 1653.594 (1629.900–1684.284) | 1649.370 (1639.430–1659.759) |
| kind: Current item | 1650.285 (1639.874–1721.886) | 1641.926 (1628.789–1653.270) |
| kind: Committed recovery | 1671.953 (1651.110–1753.168) | 1656.128 (1651.595–1700.192) |
| kind: Current terms | 2040.903 (2030.917–2093.861) | 2148.724 (2124.927–2162.089) |

### Administration and original revocation recovery

Each prepare/effect is one SDK observation in ms. Scope successors form one
successive chain; successful revocations use separate unrevoked explicit stores.
Fresh full-capacity revocations are three different cohorts, so their values are
shown individually. Every such revocation asserts `work_capacity`.

| Operation / cohort | Slots at entry | Prepare ms | Effect ms | Effect outcome |
| --- | ---: | ---: | ---: | --- |
| Scope successor | 2 | 893.681 | 2660.125 | Committed |
| Scope successor | 16 | 1351.738 | 4880.249 | Committed |
| Scope successor | 31 | 1964.703 | 7865.585 | Committed |
| Scope successor | 32 | 2018.323 | 3309.465 | Refused |
| Revocation, small explicit | 2 | 181.068 | 2105.276 | Committed |
| Revocation, explicit | 31 | 1274.927 | 7291.227 | Committed |
| Fresh revocation, full explicit | 32 | 1319.613 | 3045.442 | Refused/work_capacity |
| Fresh revocation, full kind | 32 | 1712.131 | 3650.201 | Refused/work_capacity |
| Fresh revocation, full scope chain | 32 | 1307.651 | 3013.555 | Refused/work_capacity |
| Unchanged-scope D→G | 2 | 1197.162 | 3260.642 | Committed |

The full-capacity scope successor has the same approved scope and a fresh key;
the selected `ScopeChange` rule permits this. Preparation succeeds and effect
refusal is asserted, but its specific refusal code is not captured/asserted.
The independently measured full registration/revocation cases do assert the
capacity code. Governance establishment precedes the D→G timer.

After revocation succeeds, exact original recovery still returns `Replayed`:
at 3 slots, median 218.337 ms (216.690–232.179); at 32 slots, median 1325.004 ms
(1311.831–1355.217), each **n=5** after one warmup. This exercises recovery after
current work permission has been revoked, rather than a second fresh revocation.

Initial grant activation includes new owner bootstrap: eight fresh fixtures all
committed, with individual effect times 2453.172–2592.550 ms. These are fresh-fixture
observations, not the repeated same-store read sampling above.

### Accepted pending closure at saturation

The harness interrupts at the real `Intent` callback, leaving positive accepted
original truth without a publication attempt. This is a controlled callback fault,
not a process-kill/power-loss test or the full durability fault matrix. Pending
resolution has one warmup and **n=5**; each original recovery is one SDK observation.

| Case | Slots at entry | SDK pending read median (range) ms | Original recovery ms | Recovery outcome |
| --- | ---: | ---: | ---: | --- |
| Original kind-item pending | 31 | 1615.440 (1571.884–1635.165) | 7288.044 | Committed |
| Original kind-item pending | 32 | 1645.531 (1621.276–1661.293) | 7610.987 | Committed |
| Separate pending item after independent local-check revocation | 32 | Not timed | 5080.794 | NonCommitted |

The third case revokes the independent native local-check authority outside the
timer, then closes the original never-attempted request with positive noncommit
proof. Later exact noncommit reads are verified, median 1691.935 ms
(1682.379–1779.021), **n=5**. Neither recovery creates a fresh slot or substitutes a
fresh request/basis. Closure works at the 32-slot bound in these intact, writable
stores; it does not establish success under disk failure or uncertain publication.

### Byte/codec component probes

These real full-kind-store probes have one warmup and **n=5** timed observations.
They use ordinary reads or generic JSON Values, not the complete typed native
verification path. Their operations have different inputs/preparation, so do not
sum/subtract them into an exact end-to-end profile.
The store contains 226 regular files (including the empty lock), 225 JSON records
and 9,450,694 record bytes. The selected actual kind-item intent is 226,881 bytes;
diagnostic JSON re-serialization of its authority field is 193,370 bytes, and of
its operation locator 33,324 bytes. The authority field accounts for approximately
85% of this intent's size. Subtracting that field size from the observed additional
kind-item bytes still leaves about 105,000 bytes: proof sharing alone does not
make receipts/locators negligible. This arithmetic illustrates sizing; no new
encoding or compression result is claimed.

| Component | Median ms | Observed range ms |
| --- | ---: | ---: |
| Ordinary regular-file reads | 6.996 | 6.338–7.967 |
| SHA256 of in-memory JSON bytes | 3.885 | 3.882–3.980 |
| Generic JSON Value parse | 17.428 | 16.942–18.106 |
| Canonical serialization of parsed Values | 85.029 | 83.719–93.913 |

### Beyond-cap CPU sensitivity and contention

The following **synthetic repeat** processes the same 226,881-byte kind-item intent
one at a time through generic Value decode, canonical serialization and SHA256.
Each row has one warmup and **n=5**; bytes are cumulative input processed, not a
retained store. No native count limit was changed. The loop omits authority,
uniqueness/dependency proof, native file guards, locking and durable effects; these
numbers demonstrate repeated traversal cost only, not project support beyond 32.

| Synthetic record count | Processed input bytes | Median ms | Observed range ms |
| --- | ---: | ---: | ---: |
| 32 | 7,260,192 | 63.943 | 63.660–64.837 |
| 128 | 29,040,768 | 256.416 | 253.646–261.984 |
| 512 | 116,163,072 | 1028.794 | 1022.539–1038.424 |
| 2048 | 464,652,288 | 4123.506 | 4116.261–4164.001 |

Three barrier-started pairs of native current-item reads at full kind capacity
all returned `Open`. End-to-end times include lock waiting; the probe does not
isolate exact lock-wait time or establish 8/16-caller behavior.

| Pair | Earlier completion ms | Later completion ms |
| --- | ---: | ---: |
| 1 | 1642.637 | 3279.909 |
| 2 | 1652.044 | 3318.236 |
| 3 | 1645.123 | 3320.647 |

These results are consistent with the source's exclusive blocking work-owner
lock: callers serialize successfully, rather than receive a contention refusal.
Longer verification therefore increases caller waiting as well as service cost.

## What the source explains

| Source | Causal finding |
| --- | --- |
| [Store](../../crates/engine/src/work_state/store.rs), `CAPACITY`, `inventory`, `manifest`, `reserve_intent` | 32 positive required slots, no reuse; 128 MiB inventory; complete peak reservation of `8 * MAX` plus slot publication bytes. Every inventory reads all slots, lists transaction files, reads all bytes and checks exact intent fingerprints. This is logical accounting, not physical disk reservation. |
| [Codec](../../crates/engine/src/work_state/mod.rs), `MAX`, `encode`, `decode` | Complete requests/records are bounded at 1 MiB; encoding and duplicate-safe decoding remain distinct from authority. |
| [Transaction](../../crates/engine/src/work_state/transaction.rs), `parsed`, `scan` | Every row is decoded and its original proof validated; expected decision/snapshot/terminal/checkpoint bytes are reconstructed. Equal authority proofs are deduplicated only within one scan. Pairwise request, dependency, kind-version and authority-chain searches add quadratic comparisons. |
| Same file, `execute`, `resolve`, `close_noncommit` | Preparation/read scans; fresh effects perform history resolution, admission scan, slot inventory, prepublication scan and postterminal scan. Exact checkpoints retain prior history through commit. Closure does not require a new slot, but still needs intact proof and successful I/O. |
| [Held authority](../../crates/engine/src/local_authority/work_registration.rs), `capture_proof`, `validate_current` | Original proof captures exact bounded native authority inventory into each intent. Live currentness repeatedly verifies held operating/tree guards; history uses captured proof, not today's head. Authority history growth is an independent input dimension. |
| [Durability](../../crates/engine/src/repository_durability.rs), `retain_work_history_tree_guard_v1`, `retain_exact_files_checkpoint_v1`, tree `validate` | Scan retains the whole work tree, opens files and fingerprints bytes. Validation reopens current paths and fingerprints retained/current handles. Exact checkpoints accept at most 256 paths; work tree selection is bounded at 4,096 entries, with a 256 MiB retained-tree bound. |
| [SDK](../../crates/sdk/src/work_state.rs), [usage](../../crates/sdk/docs/work-state.md), [ordinary example](../../crates/sdk/examples/work_state.rs) | SDK delegates effects inward; prepare additionally observes native basis. Saved exact original locators are consumer responsibility. CLI/example workflow durations include several operations and are not operation latency. |

Thus retained file/byte scanning, parsing/canonical reconstruction, hashing,
historical proof verification and live currentness all contribute. File operations
occur under the owner lock, so longer service time also increases contention.
Source establishes repeated work, not a CPU/I/O percentage attribution; measured
component probes must be interpreted with their stated coverage.

For the 16-slot explicit native effect, the observed total was 4,370.519 ms.
The first `BeforeIntent` notification arrived at 1,694.900 ms; `Snapshot` to
`PublicationAttempt` took 806.860 ms, and `Terminal` to `Reply` took 797.884 ms.
Those three spans total 3,299.644 ms (75.5% of the observed effect). Source places
history/currentness/retained-checkpoint verification in these spans; the middle
span also includes the publication-attempt rename. This identifies a large
verification-related cost, **not a CPU versus disk profile or a durability-only
percentage**. The short intent/decision/snapshot publication spans do not justify
removing durable writes. Component probes below bound simpler byte/codec work
without reproducing native proof and guard costs.

Raising only `CAPACITY` would also raise required slot initialization and per-read
manifest work. It would not remove the 256-path checkpoint, tree/byte limits,
quadratic searches, record bounds or revocation competition. Cap-only diagnostics
cannot establish large-project support.

The 128 MiB bound is a separate admission condition, but under current healthy
closed shapes the 32 slots should bind first: only intent, decision and snapshot
can approach 1 MiB per transaction, while publication/terminal records contain
small fixed fingerprints. This is a source-derived bound inference, not a measured
healthy near-128 MiB store. Larger authority proofs can instead hit the 1 MiB
complete-intent limit independently of transaction count. Shared proof storage for
new records must retain exact original closure and use deliberately selected
versioned identity rules; it cannot rewrite old intent preimages.

GitNexus was attempted through the project runner. `query work_state` returned
no definitions/processes, missing `Protocol`/`Category` table errors and an index
65 commits behind. File-disambiguated `context scan` and upstream `impact inventory`
and `impact scan` could not find their current symbols; risk was unknown.
`status` failed hashing analyzer identity. No index repair was performed. Direct
source and tests supply the bounded fallback; graph coverage is unclaimed.
Before commit, staged `detect-changes` identified the 12 intended files with no
indexed-symbol overlap. Comparison with `main` reported **CRITICAL** inherited
branch impact (1,205 files, 679 symbols, 300 flows); it is not attribution to this
research commit. Direct git/source verification confirms only the report, research
evidence/harness and two navigation maps changed from the admitted baseline.

## Authority availability at saturation

| Condition | Present behavior / achievable promise |
| --- | --- |
| Ordinary slot exhaustion | Fresh item, kind, scope change, D→G and revocation refuse. Existing exact reads/replay and accepted original closure retain normal proof checks. A separate administrative admission lane can avoid competition with ordinary slots. |
| Logical byte quota | Current admission reserves worst-case closure bytes before acceptance. Future admission must reserve original closure, administrative publication, metadata/index growth and temporary peaks independently of ordinary usage. Unused slot count alone proves nothing about byte availability. |
| Physical disk full or I/O failure | Logical quota is not allocated disk. A preallocated control record and closure space can reduce allocation risk, but flush/rename/media failure remains possible. No design can promise durable revocation on failed hardware; report uncertainty, retain the original locator, refuse unsupported fresh effects and retry exact supported recovery. |
| Missing/corrupt/uncertain history, owner or lock | Fail closed. A separate control lane does not authorize bypassing uncertain predecessor/currentness proof, dropping pending administration or manufacturing a noncommit. Proven independent suspension could improve availability only under explicitly selected authority and proof rules. |

A finite emergency reserve can itself exhaust through successive scope/D→G
operations, failed attempts and noncommits. Reserve admission must stop new ordinary
effects early enough to preserve closure; administration needs renewable capacity
or an explicitly bounded fail-closed maintenance posture. There is only one lineage
and no reinstatement today, so a terminal revocation could have a dedicated positive
place, but failed revocation attempts still have distinct original identities/history.
Overwriting them would violate recovery. This is a design choice, not an implemented latch.

Distinguish **restrictive safety** (no further unauthorized effects) from
**successful durable administration** (exact revocation receipt). Corrupt storage
may already enforce the first by refusing everything without proving the second.
Reserved resources improve logical saturation behavior; they cannot make an
unwritable or unprovable store truthfully return a revocation receipt.

## Credible alternatives

All alternatives retain every accepted original request, disposition, immutable
receipt and required dependency. No garbage collection or history reuse is assumed.
Performance descriptions below are algorithmic/design expectations, not alternate
backend benchmark results.

| Candidate | Scaling and concurrency | Crash, completeness and recovery | Cost, operability and trust |
| --- | --- | --- | --- |
| A. Indexed existing individual transaction files | Key/grant/kind maps replace pairwise searches; shared exact proof avoids duplicate decode. Keeping global file validation still costs total files/bytes on each call. One owner lock remains. | Derivable index can be discarded and rebuilt only from complete positive inventory. Atomically maintained authoritative index must publish acceptance/reservations/heads/terminal state coherently; stale or missing entries cannot establish absence. | Lowest layout change, readable files, retains native root/lock primitives. Helpful interim mechanism; does not alone eliminate full-tree pinning, small-file work or lifetime limits. |
| B. Positively inventoried segmented immutable history with incremental authoritative indexes | Bound mutable tail/segment size; indexed request/dependency lookups and shared exact proof reduce work to selected dependencies plus tail. Serialized writes initially; concurrent reads require a proven coherent generation. Total exact history still grows. | Required segment identities/counts and durable head/checkpoint prevent missing segments from becoming absence. Acceptance and terminal/noncommit transitions need exact ordering and uncertain-tail recovery. Checkpoint commits exact verified cut; rebuild streams complete original history. | Fewer files and reusable native durability, explicit human-readable export. More bespoke framing, index/checkpoint publication and recovery engineering. A checksum chain alone does not detect deletion of the latest head or prove all unexamined bytes unchanged. |
| C. Embedded transactional owner database with exact canonical-byte history | Transactional unique keys and indexes serve requests/current heads; immutable proof blobs can be shared. Database commit can co-publish admission/history/head. A local SQLite contender supports concurrent readers with one writer in WAL mode. | Application must persist positive acceptance before possible effects and retain attempted/terminal/noncommit meaning; rollback is not original-request noncommit proof. Required owner/generation identity and integrity checks are still needed. Missing/corrupt rows or substituted DB/WAL cannot become absence. | Established transaction machinery, fewer files, new dependency/schema/VFS and integrity boundary. Exact bytes/identities remain blobs, with explicit readable export. SQLite locks do not replace held Engine authority or retained-root substitution checks. |

SQLite's official [atomic-commit description](https://www.sqlite.org/atomiccommit.html)
explains crash consistency and filesystem/flush assumptions. Its
[WAL documentation](https://www.sqlite.org/wal.html) documents same-host use,
one writer, read/write concurrency and checkpoint constraints; long readers can
delay checkpoint progress. The [synchronous pragma](https://www.sqlite.org/pragma.html#pragma_synchronous)
distinguishes power-loss durability: WAL `NORMAL` can lose acknowledged commits;
a contender must use an explicitly justified durable setting, not benchmark it
as equivalent to current durable publication. [Corruption guidance](https://www.sqlite.org/howtocorrupt.html#_unlinking_or_renaming_a_database_file_while_in_use)
describes rename/unlink and journal pairing hazards. [Result codes](https://www.sqlite.org/rescode.html#full)
separate full storage, I/O error and corruption. No SQLite version, Rust binding,
journal mode or production dependency is selected or performance-tested here.
A future WAL contender must verify its actual bundled version includes the
documented [WAL-reset correction](https://www.sqlite.org/wal.html#the_wal_reset_bug)
(3.51.3 or an identified backport), rather than assume its binding is sufficient.

Administrative admission/recovery resources are **orthogonal to A/B/C**. Merely
putting administration in another file/database risks a split transaction and
inconsistent current head. The selected owner must coherently consume restrictive
authority state before allowing ordinary effects, preserving existing pending
suspension and original recovery.

### Index and checkpoint trust boundary

An acceleration cache may never supply acceptance, absence or noncommit proof.
An authoritative index needs positive complete inventory, exact owner/generation
binding, atomic publication and verified dependency linkage. Stale/missing index:
refuse affected operations or rebuild under exclusive ownership from an intact
positive inventory. Missing history/checkpoint: uncertainty, not automatic empty
initialization. Rebuild does not infer missing originals from files that happen
to remain.

Current scans detect unrelated history damage on the next operation as well as
selected dependency damage. A bounded point read plus a hash stored elsewhere
cannot independently prove every unexamined byte is unchanged. Neither SQLite nor
segments remove that fact. Before replacing whole-tree validation, explicitly
prove an integrity mechanism preserving the selected mutation/substitution threat
boundary, or return a human decision about detection scope/timing. This report
does **not** approve silently changing global detection to eventual audit. First
use/reopen verification, coherent sealed history, bounded active-tail verification
and explicit full audits are candidates whose proof must be evaluated.

## Recommendation and decisions

**Candidate direction: B, an Engine-owned positively inventoried segmented history
with transactional authoritative indexes, shared immutable exact authority proof
and protected administrative/accepted-closure resources.** Preserve current
semantic invariants and native authority/physical-root guards. Keep initial writes
serialized; first establish bounded owner operations before adding read concurrency.
Observed read growth, multi-second fresh mutations and shared administrative
saturation support investigating these mechanisms together. They do not establish
the speed or correctness of an alternate backend.

This direction addresses repeated whole-history work and small-file populations
without immediately replacing the proven native root/lock boundary. A is a useful
lower-complexity alternative if the human favors an interim file layout: keyed
checks and proof reuse may help, but a cache or raised constant alone cannot meet
the proposed long-lived envelope. C is a credible alternative if reducing bespoke
publication/recovery machinery outweighs a new database/VFS boundary. It is deferred,
not rejected for speculative speed; neither B nor C has a measured full-owner
implementation here. The recommendation is conditional on proving completeness,
integrity/currentness and original-prefix recovery. A failed proof must return for
selection, not weaken guarantees to meet latency.

The bounded next implementation outcome, **if separately selected and authorized**,
would replace only work-state storage/admission/verification mechanisms and expose
the same meaningful native/SDK operations: sustained registration, exact/current
reads, authority administration and original recovery. No hierarchy, eligibility,
closing, CR, broad IAM or consumer expansion. This is an outcome boundary, not
packet design or approval.

### Proposed measurable acceptance targets

Targets are design inputs for human selection, **not current performance promises**.
Use representative Windows and Unix local SSDs, fixed native-authority cohorts,
ordinary/default SDK features, release profile and documented stacks. Populate
outside timing; report end-to-end prepare/save/invoke separately, durable settings
and at least 200 read samples / 50 fresh mutations per cohort, with empirical
median/p95 and complete failures, not small-sample confidence claims.

| Area | Proposed target / measurement boundary |
| --- | --- |
| Capacity envelope | Demonstrate 10,000 and 100,000 mixed accepted transactions; 10% authority/kind/noncommit history sensitivity, plus separately grown native authority histories. One million remains a long-lived design sensitivity, not a first implementation promise. No silent lifetime ceiling or history reuse. |
| Warm exact/current reads | Median ≤50 ms, empirical p95 ≤100 ms at 10,000 and 100,000 transactions, absent lock contention. Include original committed replay and definition-backed dependencies. |
| Preparation | Median ≤150 ms, p95 ≤250 ms for native and SDK separately; label basis observation, locator persistence and serialization costs. |
| Durable mutation / pending closure | Median ≤250 ms, p95 ≤500 ms absent contention for register, kind publication, scope, D→G and supported original continuation. Correct refusal/uncertainty also counted; never substitute refusal time for successful latency. |
| Revocation at ordinary saturation | With valid current authority and working storage, new revocation remains admissible and p95 ≤500 ms when ordinary admission is full. Each accepted closure has protected peak resources. Saturate administrative reserve too: explicit restriction/maintenance refusal before proof obligations can be lost; no unsupported competing administration. |
| Resources | Routine operation additional memory ≤64 MiB and ≤128 simultaneous retained file handles; recovery/rebuild memory ≤256 MiB through streaming. Bound active tail, maximum selected proof/dependency closure and closure reserve explicitly; retain current complete-record bounds unless separately changed. These numbers are proposed budgets, not measured maxima. |
| Reopen / rebuild | With intact authoritative checkpoint, recover a bounded active tail and become usable within 5 s at 100,000 transactions on selected SSD hardware. Full verification/rebuild may scale with all bytes: report progress, bytes/s and total time, remain exclusive and fail closed until proof complete; do not call it a 5 s point operation. |
| Contention | Exercise 2/8 concurrent local callers and 16-caller sensitivity; distinguish service and lock-wait latency. No starvation or lost/duplicate heads/items/receipts. The initial owner remains serialized; admission must return a bounded wait/refusal consistent with original acceptance state. |
| Crash / damage | Every new acceptance, index/head, checkpoint/segment and terminal boundary: interrupted process, lost reply, missing/stale/corrupt index, missing segment/original/dependency, copied root/lock, storage full and failed durable write. Exact committed replay, positive noncommit and uncertainty remain distinct. Verify unrelated corruption detection under the explicitly retained or selected integrity policy. |

Do not make the numbers defaults merely because they are convenient to test. The
source shows authority proofs and guards can dominate independently of work-state
layout; meeting these targets may require a bounded exact-proof reuse mechanism,
not only an index. Hardware power-loss durability needs separately appropriate
evidence; process-kill testing does not certify a device's flush behavior.

### Exact human selections still needed

1. Accept or revise the workload/resource/latency envelope, particularly first
   support at 10,000/100,000 transactions versus one-million lifetime ambition.
2. Select B as the candidate direction, A as an interim mechanism, or C for a
   transactional database investigation; no database/library/layout is already selected.
3. Select the saturation promise: revocation and accepted closure protected from
   ordinary capacity, with explicit administrative exhaustion/maintenance behavior.
   Physical I/O failure remains truthful uncertainty, not guaranteed durable receipt.
4. Require a demonstrated bounded integrity proof retaining the current detection
   boundary, or explicitly decide a changed detection scope/timing. No relaxed
   integrity policy is selected here; an index must not implicitly change it.
5. Choose treatment of actual retained stores from the options below, including
   unresolved originals. Separately authorize a concrete bounded plan after these
   selections. No production implementation is selected by this report.

## Greenfield and actual retained truth

Public API compatibility is not permanent by default. Actual retained canonical
bytes, identities, receipts, roots and original recovery remain evidence that a
new representation must consider. Three research choices:

1. Keep existing stores read/recover-only through their native owner while new
   stores use the selected representation. This needs an explicit old/new writer
   exclusion boundary and original locator routing; it is not a permanent old API promise.
2. Rebuild an index over the unchanged physical originals, keeping identity and
   recovery with their original owner. This preserves bytes but cannot remove old
   acceptance/closure constraints by index creation alone.
3. Explicitly reimport verified complete original history into a new owner with a
   provable old-root/old-receipt mapping and exclusive cutover. Copying JSON alone
   cannot prove physical identity or pending prefixes. Missing/uncertain stores
   are not silently repaired or merged into fresh authority.

Choosing fresh stores for future development does not authorize erasing actual
retained state. No captured historical cohorts or user stores were used as
disposable benchmark stores. Migration tooling, archival and garbage collection
are outside this research.

## Verification and limits

The locked release `all` run exited **0**. The retained
[raw samples](work-state-capacity-and-performance-2026-10-10/results.jsonl) contain
593 complete JSONL rows: **588 timed observations, 160 summary groups and five
metadata rows**. The [summary](work-state-capacity-and-performance-2026-10-10/summary.json)
matches an independent arithmetic check of every sample count, median, extrema,
byte range and metadata row. Every repeated single-caller read/component/synthetic
group has n=5; initial activation and full revocation pool distinct fixtures, explicitly
identified above. Raw SHA256:
`f641fd72e7504b24b81ebb6177fc1c0f9f18bd8e3152b7fc2747d0a65347eac2`.

All 184 dependency package name/version/source triples match the workspace lock;
only the research package was added to its smaller retained closure. No Rust
source changed after the accepted executable build. Scoped Rust formatting and
summary Python syntax checks pass. Document links, intended git scope, diff checks
and one focused causal review complete the commit verification.
The focused reviewer returned **PROCEED**, with no unresolved P1 Critical or P2
Required findings. Its optional P4 sampling-description nit was resolved by
qualifying the n=5 groups as single-caller; contention remains three pairs.
The review permits this research commit, not design selection or implementation.
Existing product suites are precedent; they are not rerun to turn a
research commit into a new product certification. Measurements and options do
not establish Unix execution, network-filesystem support, power-loss hardware
behavior, customer workloads or an implemented scalable backend.
