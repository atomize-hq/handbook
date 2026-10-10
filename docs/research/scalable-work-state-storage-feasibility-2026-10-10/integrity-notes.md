# Integrity feasibility: source findings and proposed mechanisms

Research note, 2026-10-10, source baseline `889d9b24f0d3be33169c14951d4336dd86ee03e2`.
This is analysis for the scalable-storage investigation, not approval of a
backend, format, changed detection boundary, or implementation. The
[structured-data standard](../../specs/handbook-contract-membrane/structured-data-format-standard.md)
has proposed general defaults, fixed Contracts JSON, and controlling existing
owner/versioned contracts. No product source or retained project store was changed.

GitNexus MCP tools were unavailable. CLI 1.6.12 `query work_state --repo handbook`
returned empty results because all FTS queries failed with missing `Protocol` /
`Category` tables; it reported an index 66 commits behind HEAD. `context scan
--repo handbook` could not find the symbol. These findings use direct source and
focused existing tests; graph coverage and a new native corruption experiment
are not claimed. This note introduces no code symbol requiring impact analysis.

## Controlling meaning

The [completed slice](../../specs/handbook-contract-membrane/slices/work-compositions-and-kinds/README.md#development-compatibility-and-retained-truth)
distinguishes revisable greenfield APIs from retained actual truth. Exact item
identity, submitted/effective composition, original grants/kinds/receipts,
accepted requests, native basis, reservations and original bytes cannot be
erased or silently rebound. Packet 1 selects one unbranched authority lineage,
explicit human scope successors, unchanged-scope D→G with fresh activation,
pending administrative suspension and no reinstatement. Kind establishment is
separate from registration permission. These constrain all three alternatives;
they do not prescribe a database or migration platform. See
[Packet 1](../../specs/handbook-contract-membrane/slices/work-compositions-and-kinds/packet-1.md#original-request-recovery-and-transitions),
[Packet 2](../../specs/handbook-contract-membrane/slices/work-compositions-and-kinds/packet-2.md),
and the [selected history/currentness amendment](../../specs/handbook-contract-membrane/semantic-model/amendments/work-progression-kinds-and-relations.md#selected-revision-currentness-and-history-rules).

## What the current native owner actually observes

| Proposition | Exact source and limit |
| --- | --- |
| Physical owner/root/lock binding | [`WorkLock::validate`](../../../crates/engine/src/work_state/store.rs#L178), lines 178–194, requires original native lock fingerprint, exact owner/anchor/bootstrap bytes and root fingerprint. [`store::open`](../../../crates/engine/src/work_state/store.rs#L549), lines 549–570, checks initialized owner proof and acquires an **existing** lock. |
| Positive complete work inventory | [`inventory`](../../../crates/engine/src/work_state/store.rs#L197), lines 197–337, admits only a closed partition/transaction shape, reads every retained transaction file, checks slot-declared intent hashes and rejects missing/conflicting/undeclared transactions. [`manifest`](../../../crates/engine/src/work_state/store.rs#L348), lines 348–426, requires all 32 exact slots and each occupied slot's exact displaced empty predecessor. |
| All work-history bytes stable during a scan | [`scan`](../../../crates/engine/src/work_state/transaction.rs#L321), lines 321–382, retains the entire work-history tree, parses every row and revalidates tree, metadata and lock. [`retain_work_history_tree_guard_v1`](../../../crates/engine/src/repository_durability.rs#L788), lines 788–800, recursively selects `work-state/v1`. [`RetainedRepositoryTreeGuardV1::validate`](../../../crates/engine/src/repository_durability.rs#L244), lines 244–287, reopens paths, checks native identities/versions and rehashes both retained and current non-lock file contents. |
| Historical native authority | [`parsed`](../../../crates/engine/src/work_state/transaction.rs#L233), lines 233–319, binds owner/request/check/basis, computes a proof identity, verifies each **distinct exact** proof once per scan, and derives exact decision/snapshot/checkpoint/terminal/noncommit bytes. [`validate_historical`](../../../crates/engine/src/local_authority/work_registration.rs#L236), lines 236–395, uses copied original owner history and expressly consults no live filesystem/Charter/terms. [`validate_shape`](../../../crates/engine/src/local_authority/work_registration.rs#L501), lines 501–585, hashes each copied file and binds the admitted governance generation/relation. |
| Dependencies and competing state | `scan`, lines 352–379, requires committed original grant and kind dependencies for **all** applicable rows; [`validate_authority_history`](../../../crates/engine/src/work_state/transaction.rs#L387), lines 387–499, enforces one closed chain, no branch/restoration and at most one pending administration. [`current_grant`](../../../crates/engine/src/work_state/transaction.rs#L558), lines 558–589, refuses a predecessor with a non-noncommitted successor/revocation. |
| Through-publication coherence | [`execute`](../../../crates/engine/src/work_state/transaction.rs#L1333), lines 1333–1555, uses native authority guards before the work lock; pins all other immutable work files and later manifest files; rechecks current native basis, grant/kind and pinned bytes immediately before and after terminal installation. [`with_work_registration_establishment`](../../../crates/engine/src/local_authority/work_registration.rs#L150), lines 150–189, holds original native locks across bootstrap, work-lock acquisition and terminal publication. |

The supported **next native scan** detects ordinary unrelated work-history
corruption/loss too, not only damage to the requested item's dependency closure.
It is not a claim to inspect every byte of every other owner's live store on a
historical read. Original native authority is retained as copied evidence. A
new effect additionally checks the separately current native authority tree.
Exact proof deduplication already exists **within each scan**; repeated identical
proofs still undergo parsing/canonical proof-identity work and byte observation.

## Damage and threat boundary

| Case | Current guarantee / qualification |
| --- | --- |
| Selected or unrelated work file changed | Closed shape, exact derived records, slot intent hashes, copied proof hashes and tree validation fail closed for observed damaged evidence. It need not yield the same API disposition everywhere: historical `resolve` converts scan errors to `Uncertain`; some preparation/current-read APIs return a refusal. |
| Whole transaction deleted/truncated | Surviving positive slot declaration makes loss a missing required original, not absence. Missing records, unsupported prefixes, conflicts and malformed bytes fail closed. Existing tests exercise whole item/kind/grant loss, slot erasure, corrupted exact records and missing attempt proof. |
| Root or lock substituted | Native identities bind the retained path and original lock generation. A copied replacement root/lock cannot become the original merely because JSON hashes match. Supported native root paths are normalized and link-free; live path/identity checks remain required. |
| Partial rollback, stale manifest/head | Resetting only an occupied slot conflicts with displaced predecessor and/or undeclared transaction; dropping only a newer grant leaves declaration/dependency evidence. The work owner derives its head from a validated chain rather than selecting a newest file. |
| Coherent hostile rollback/rewrite between opens | **No general tamper-proof or monotonic-history guarantee found.** Hashes and approval/principal fields are local, not authenticated against an independent secret or external durable generation. Counterexample inferred from source: restore an earlier valid slot in place, remove its later displaced predecessor and transaction, preserve original root/lock and all earlier bytes, then reopen. Fresh guards observe that internally consistent earlier inventory; there is no independent persisted generation to prove that later history once existed. This is an analytical counterexample, not a new exercised attack. |
| During-operation out-of-band changes | Native handles, identities/versions, repeated byte reads and stable tree/path guards detect or, on Windows, can prevent supported substitutions. They do not establish security against a hostile kernel or storage device fabricating reads/metadata. |
| Crash and physical I/O failure | Durable write/rename primitives flush files and affected directories, and uncertainty preserves original reservations when application or flush outcome is unproven. This relies on the platform/storage honoring successful flushes; process-kill coverage is not a power-loss proof. Neither current code nor any candidate can promise successful durable revocation through physical I/O failure. |

Sources for the last boundaries: [`RepositoryExclusiveLockV1::validate_at`](../../../crates/engine/src/repository_durability.rs#L324)
and fingerprint, lines 324–383;
[`RetainedRepositoryRootV1::open_exact`](../../../crates/engine/src/repository_durability.rs#L599)
and physical fingerprint, lines 587–671;
[`retained_file_fingerprint`](../../../crates/engine/src/repository_durability.rs#L1431),
lines 1431–1484; Windows durable file writes / no-replace publication, lines
2500–2580, and Unix writes / rename flushes, lines 2146–2222.
The [slice](../../specs/handbook-contract-membrane/slices/work-compositions-and-kinds/README.md#unified-owner-and-permission-boundaries)
explicitly retains advisory approval/principal/delegation claims without adding
authentication/host enforcement. Existing evidence is in
[work-state integrity tests](../../../crates/sdk/tests/work_state_integrity.rs#L90),
[work-kind integrity tests](../../../crates/sdk/tests/work_kind_integrity.rs#L204)
and [durability tests](../../../crates/sdk/tests/work_state_durability.rs#L373).

## Why bounded point lookup cannot preserve global byte observation by itself

Consider a lookup that reads only its index/head, selected record, dependency
proofs and a fixed or logarithmic number of manifest/Merkle-path nodes. Compare a
healthy store with one differing only in an unrelated retained payload byte that
the lookup never observes. All inputs the algorithm observes are identical, so
it cannot distinguish the stores. The same counterexample defeats a stored hash
chain tip, Merkle root, segment checksum or cached fingerprint: the stored
commitment remains unchanged while an unexamined byte is damaged.

A trusted commitment proves membership/value for **bytes actually supplied and
hashed**; it does not prove that all unexamined physical bytes still exist and
match. Detecting arbitrary damage to every retained byte at the next operation
therefore needs observation proportional to the retained byte population under
today's local-storage model. Inventory loss detection may need fewer byte reads,
but still cannot establish unexamined content integrity. Trusted storage,
authenticated change notification or a continuously verified external service
could supply additional observations/assumptions; none is currently selected.
Scrubbing at reopen or periodically changes detection timing, not this argument.

## Concrete mechanisms worth testing, without selecting them

| Alternative | Proposed selected-dependency/commit mechanism | Required assumptions and counterexample |
| --- | --- | --- |
| A: indexed individual immutable files | Exact payload/proof blobs keyed by their byte identity; request/item/kind/reservation/head index bound to an explicit owner generation; positive declaration of every accepted original; immutable predecessor retained. Durable original acceptance precedes candidate/terminal publication. A published index generation names exact files and terminal state. Rebuild a missing derived index under the original owner lock from authoritative accepted history; refuse damaged declarations rather than call them absence. | Preserve physical root/lock and original native guards. A point read must verify the selected file, proof bytes, lineage and authoritative index/generation; an absent key requires trustworthy complete index coverage. Unrelated-file bit flip survives untouched index. Keeping current global observation adds a full file/byte pass; file-open and proof repetition may still be reduced. |
| B: immutable segments and generation-bound indexes | Length-framed exact records with checksum, sequence and request/state binding; sealed segment manifest names digest, byte length and sequence coverage; current accepted boundary and generation are durably published with index/head state. A terminal record, not mere appended candidate bytes, establishes the effect. Active-tail recovery may discard only a tail positively beyond the authoritative accepted/committed boundary. Missing segment or committed boundary beyond available bytes is damaged/uncertain. | Segment checksums verify a whole segment only when read/hashed; bounded record/Merkle proofs verify selected data. A stale/missing index rebuilds from authoritative segments, reading them. Torn append or old head cannot silently be used to infer noncommit. A damaged unrelated sealed segment remains undetected by selected lookup; a cached seal digest does not change this. Atomic cross-file publication/recovery requires an explicit prefix protocol. |
| C: embedded SQLite owner | Store exact original payload/proof bytes as BLOBs, immutable request identities, dependencies, reservation and terminal state. One durable acceptance transaction reserves the exact original; a later terminal transaction atomically changes terminal/head/index state after original authority revalidation. Unique keys/expected predecessor and transactional constraints serialize competing effects. Recover a lost reply by the original request, checking its exact bytes and terminal row. A failed/unacknowledged COMMIT remains uncertain until native reopen/recovery proves its result. | SQL transactions do not confer original native authority or automatically retain the physical root/lock/path binding: those guards and SQLite's DB/WAL/SHM opening behavior need a deliberate compatible design. Validate selected BLOB byte hashes plus semantic original proof; database structure checks alone do not establish those bytes. An unrelated payload bit flip or a valid SQL deletion/coherent earlier DB image can leave selected lookup and structural checks successful. Global observation still needs all authoritative payload/proof bytes and owner semantic constraints checked. |

These mechanisms are design proposals, not claims about the standalone research
prototypes. Each prototype must separately declare what it models, implements,
verifies and omits. A fast indexed record append is not a fully authority-admitted
native work registration. Proof reuse should retain an exact immutable proof
object and preserve old canonical preimage domains; replacing an old intent's
embedded proof with a reference would rewrite its bytes and is not a permissible
optimization of that retained record. New versioned records could reference the
exact object under an expressly selected contract.

SQLite primary sources verified 2026-10-10:
[WAL](https://www.sqlite.org/wal.html) documents one concurrent writer, same-host
use, committed persistent WAL state and its necessary retention. Use WAL
`synchronous=FULL`, not `NORMAL`, for durable-commit comparison: FULL adds a WAL
sync per commit; NORMAL can lose committed transactions after power loss. WAL's
reset race is fixed in 3.51.3 and later, with backports 3.44.6 and 3.50.7; affected
versions include 3.7.0–3.51.2 with competing connections writing/checkpointing.
[PRAGMA documentation](https://www.sqlite.org/pragma.html#pragma_integrity_check)
describes structural/constraint/index checking; it excludes foreign-key errors
(separate `foreign_key_check`). `quick_check` is O(N), omits UNIQUE/index-content
checks; `integrity_check` is O(N log N). Neither establishes exact owner semantics
or authenticates history. [Atomic-commit assumptions](https://www.sqlite.org/atomiccommit.html#_hardware_assumptions)
include functioning flush/storage behavior. Actual linked library version and
effective settings must be recorded, not inferred from a package version.

## SQLite and native physical guards: unresolved integration gap

**Physical root/lock binding appears compatible in principle; equivalent
through-publication protection against out-of-band physical changes is not
established with the default Windows VFS.** This is a source-based constraint,
not a new native integration experiment or a causal-review verdict.

Existing native checkpoints pin **unchanged independent files** and compare
their original identity, version, length and content at repeated checkpoints
(`repository_durability.rs`, lines 244–287). The work transaction writes new
files while pinning the other originals (`transaction.rs`, lines 1349–1372).
A SQLite terminal transaction legitimately mutates a shared DB/WAL and may
checkpoint/reset its WAL; wrapping those files in the existing immutable
checkpoint would reject legitimate owner activity. Simply allowing changed
metadata, or refreshing a hash after the owner writes, does not prove that no
out-of-band writer also changed bytes. A cached SQL reread is not independent
physical evidence that the retained dependency bytes remained intact.

The native retained root already pins its Windows ancestor directories with
no delete sharing (`repository_durability.rs`, lines 1829–1847, 1869–1893).
The existing work lock retains its parent chain and exact lock fingerprint
(lines 1227–1244 and `store.rs`, lines 178–194). A candidate can retain those
guards through a SQL transaction and require an exact DB identity belonging to
the initialized owner. It would need an owner-side binding for SQLite's actual
opened handles, safe handling of first creation/reopen, and admissible DB/WAL
lifecycles; none is present in the current work-state owner. Stable parent and
DB identities establish location/generation, not unchanged mutable contents.

The official [SQLite 3.53.1 Windows source](https://github.com/sqlite/sqlite/blob/version-3.53.1/src/os_win.c)
was checked for the selected research runtime. `winOpen`, lines 5046–5065,
uses read/write sharing by default; a separate URI `exclusive` parameter sets
sharing to zero. `SQLITE_OPEN_EXCLUSIVE` means create-new, not exclusive access.
`winLock`, lines 3012–3025 and 3182–3206, uses protocol byte ranges.
`SQLITE_FCNTL_WIN32_GET_HANDLE`, lines 3432–3436, exposes the native main-file
handle. SHM creation uses `winHandleOpen` internally, lines 3854–3903 and
4015–4020, so a shim wrapping only VFS `xOpen` would not cover that opening.
These findings confirm the same default-sharing boundary found in 3.51.3.

Microsoft's [CreateFile sharing contract](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew)
explains why denied delete sharing can bind an open file against rename, while
write sharing permits another writable handle. Its
[LockFileEx contract](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfileex)
applies to locked byte ranges, permits access through mapped views, and blocks
an owner's separate handle too. Thus neither SQLite protocol locks nor a
naively added all-file native lock prove the required raw-byte coherence.

Two bounded analytical mechanisms merit a future integration feasibility check;
neither is implemented or selected here:

1. **Serialize one owning connection under the original guards and work lock.**
   A purpose-bound VFS could open the DB/WAL without external writable/delete
   sharing, bind the actual handles to the retained owner/root, and retain them
   through admission/terminal/verification. Competing owners wait on the same
   work lock before opening SQLite. Cover or deliberately eliminate SHM, retain
   correct file flush/recovery behavior, and verify that all required reopening,
   checkpoint and close operations remain possible. Windows source's optional
   zero-sharing branch is evidence of an OS mechanism, not proof that a URI
   switch alone solves the DB/WAL/SHM lifecycle. `PRAGMA locking_mode=EXCLUSIVE`
   and `SQLITE_OPEN_EXCLUSIVE` are not substitutes for this proof. The official
   [VFS interface](https://www.sqlite.org/vfs.html) permits custom implementations;
   the proposed wrapper is additional maintenance/dependency complexity.
2. **Verify physical expected writes and protected ranges.** A verifying VFS
   could bind handles/paths, retain a trusted in-memory baseline for the protected
   physical bytes, check them before overwrites, track only its own exact
   `xWrite`/truncate changes, and reread protected bytes at the commit boundary.
   It must address SQLite page caching, mappings, page relocation, WAL reset,
   checkpoint, auxiliary files and concurrent readers. Merely hashing VFS reads
   or assuming every dependency is reread does not cover those paths. This
   proposal may retain substantial observation cost and has no compatibility,
   complexity or overhead proof yet. Independent immutable proof sidecars are
   another possible boundary, but they change the physical design and require
   an explicit cross-file commit protocol.

The smallest unresolved decision is whether one concrete integration can retain
the selected native physical binding and dependency coherence while preserving
truthful accepted/pending/terminal recovery. Before calling SQLite a feasible
full owner, test that integration's ordinary SQL writes and its raw-write,
mapped-write, rename/reparse, missing WAL, checkpoint/reset and process-death
cases under the original guards. A passing standalone durable SQL prototype
does not answer this gap. This research condition precedes backend selection;
it authorizes no implementation packet and silently relaxes no safety boundary.

## Required state-outcome obligations

The exact source contracts are [`resolve` and `execute`](../../../crates/engine/src/work_state/transaction.rs#L1232),
lines 1232–1318 and 1410–1574, and
[`close_noncommit`](../../../crates/engine/src/work_state/transaction.rs#L1577),
lines 1577–1627. Alternatives must retain these discriminants:

- Original accepted intent without terminal is pending only with intact supported
  prefix. Completion needs that original request/basis/rule and original usable
  grant/kind. A newer grant or fresh key cannot repair uncertainty.
- Exact terminal plus matching original records/proof replays before checking
  current permission; lost reply never implies a second effect or reinstatement.
- Missing/conflicting attempt checkpoint is uncertain. Proven never-attempted
  accepted original can close noncommit under complete owner evidence; an attempted
  prefix without exact terminal proof cannot be relabeled noncommitted by timeout.
- Missing/stale derived index can be rebuilt from intact authority; loss of
  authoritative declaration/head/history remains damage. “Index says absent” is
  insufficient if coverage/current generation has not been proven.
- Fresh ordinary saturation, protected administration/closure capacity and
  physical exhaustion are different. Today **all** fresh operations share 32
  lifetime slots; new revocation refuses at capacity, while accepted original
  completion requires no new slot. Protected fresh-administration reserves are a
  proposed capacity policy requiring explicit design, not an existing guarantee.
  Exhausted administration capacity must refuse truthfully; accepted closure still
  depends on writable physical storage and its reserved peak space assumptions.

## Consequential choices still requiring the human

The evidence supports separating exact selected-dependency verification,
through-commit coherence and crash recovery from next-operation inspection of
unrelated retained history. It does **not** authorize changing any of them. A
storage design must choose explicitly between preserving global observation on
every operation (still proportional to history, with possible constant-factor
improvements) and selected point verification plus a declared reopen/scrub damage
detection protocol. Stronger authenticated antirollback storage would be a
different threat-model choice, not a free property of indexes, hash chains or
SQLite. These are policy/design decisions; this note selects no backend or scope.
